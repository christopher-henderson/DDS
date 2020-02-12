use image::{RgbaImage, ImageFormat};
use crate::api;

static MLB_LOGO_LARGE_BYTES: &[u8] = include_bytes!("../../assets/mlb_logo_large.jpg");
static MLB_LOGO_SMALL_BYTES: &[u8] = include_bytes!("../../assets/mlb_logo_small.jpg");

lazy_static! {
    static ref MLB_LOGO_LARGE: RgbaImage =
        image::load_from_memory_with_format(MLB_LOGO_LARGE_BYTES, ImageFormat::JPEG)
            .unwrap()
            .into_rgba();
    static ref MLB_LOGO_SMALL: RgbaImage =
        image::load_from_memory_with_format(MLB_LOGO_SMALL_BYTES, ImageFormat::JPEG)
            .unwrap()
            .into_rgba();
}

pub struct Schedule {
    pub games: Vec<Game>,
    cursor: usize,
}

impl Schedule {
    const PAGE_SIZE: usize = 5;

    pub fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.cursor < self.games.len() - 2 {
            self.cursor += 1;
        }
    }

    pub fn has_more(&self) -> bool {
        self.cursor < self.games.len() - Self::PAGE_SIZE
    }

    pub fn has_less(&self) -> bool {
        self.cursor > Self::PAGE_SIZE - 1
    }

    /// Returns the list of game snippets for the current page. Each page has five games on it.
    ///
    /// E.G. If, there are are 14 games and we are focusing on game index 7, then this function will
    /// return games indices 5, 6, 7, 8, and 9 with 7 being the Snippet::Large variant.
    pub fn page(&mut self) -> Vec<Snippet> {
        let page = self.cursor / Self::PAGE_SIZE;
        // The left most snippet of this page.
        let left = page * Self::PAGE_SIZE;
        // The right end of the page can fall off if the map if we're on the last page.
        let right = match left + Self::PAGE_SIZE {
            right if right < self.games.len() - 1 => right,
            _ => self.games.len() - 1,
        };
        // The cursor may be 7, but the focus of this page is index 2.
        let page_focus = self.cursor % Self::PAGE_SIZE;
        // Sorry the extra parenthesis here, rustc thought that we were returning a &mut rather
        // than accessing self.games as a &mut.
        (&mut self.games)[left..right]
            .iter_mut()
            .enumerate()
            .map(|(index, game)| {
                if index == page_focus {
                    // If the underlying resource hasn't come in over the network yet, then this
                    // is the point where we decide to default to the appropriate size of the MLB logo.
                    Snippet::Large(
                        game.large.get().unwrap_or(&*MLB_LOGO_LARGE),
                        game.headline.as_str(),
                        game.subhead.as_str(),
                    )
                } else {
                    Snippet::Small(game.small.get().unwrap_or(&*MLB_LOGO_SMALL))
                }
            })
            .collect::<Vec<Snippet>>()
    }
}

pub enum Snippet<'a> {
    Small(&'a RgbaImage),
    Large(&'a RgbaImage, &'a str, &'a str),
}

impl From<api::Schedule> for Schedule {
    fn from(mut schedule: api::Schedule) -> Self {
        let mut games = vec![];
        for game in schedule.dates.pop().unwrap().games.into_iter() {
            games.push(Game {
                headline: game.content.editorial.recap.home.headline.clone(),
                subhead: game.content.editorial.recap.home.subhead.clone(),
                large: Photo::new(game.content.editorial.recap.home.photo.cuts.large.src),
                small: Photo::new(game.content.editorial.recap.home.photo.cuts.small.src),
            });
        }
        Schedule { games, cursor: 0 }
    }
}

pub struct Game {
    pub headline: String,
    pub subhead: String,
    large: Photo,
    small: Photo,
}

pub struct Photo {
    photo: Option<RgbaImage>,
    channel: crossbeam_channel::Receiver<RgbaImage>,
}

impl Photo {
    pub fn new(src: String) -> Photo {
        let (tx, rx) = crossbeam_channel::bounded(1);
        tokio::task::spawn(async move {
            let https = hyper_tls::HttpsConnector::new();
            let resp = hyper::Client::builder()
                .build::<_, hyper::Body>(https)
                .get(src.parse().unwrap())
                .await
                .unwrap();
            let buf = hyper::body::to_bytes(resp).await.unwrap();
            let img = image::load_from_memory_with_format(&buf, ImageFormat::JPEG)
                .unwrap()
                .into_rgba();
            tx.send(img).unwrap();
        });
        Photo {
            photo: None,
            channel: rx,
        }
    }

    pub fn get(&mut self) -> Option<&RgbaImage> {
        if  self.photo.is_some() {
            return self.photo.as_ref();
        }
        match self.channel.try_recv() {
            Ok(image) => {
                self.photo = Some(image);
                self.photo.as_ref()
            }
            _ => None,
        }
    }
}