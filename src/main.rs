#[macro_use]
extern crate lazy_static;

use image::{ImageFormat, RgbaImage};
use piston_window::{EventLoop, Glyphs, ReleaseEvent, Transformed};

mod api;

static BACKGROUND_BYTES: &[u8] = include_bytes!("../assets/background.jpg");
static MLB_LOGO_LARGE_BYTES: &[u8] = include_bytes!("../assets/mlb_logo_large.jpg");
static MLB_LOGO_SMALL_BYTES: &[u8] = include_bytes!("../assets/mlb_logo_large.jpg");
static CANARY_BYTES: &[u8] = include_bytes!("../assets/canary.jpg");
static LEFT_ARROW_BYTES: &[u8] = include_bytes!("../assets/left_arrow.png");
static RIGHT_ARROW_BYTES: &[u8] = include_bytes!("../assets/right_arrow.png");
static FONT: &[u8] = include_bytes!("../OpenSans-Bold.ttf");
static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
static PADDING: f64 = 27.5;

lazy_static! {
    static ref BACKGROUND: RgbaImage =
        image::load_from_memory_with_format(BACKGROUND_BYTES, ImageFormat::JPEG)
            .unwrap()
            .into_rgba();
    static ref MLB_LOGO_LARGE: RgbaImage =
        image::load_from_memory_with_format(MLB_LOGO_LARGE_BYTES, ImageFormat::JPEG)
            .unwrap()
            .into_rgba();
    static ref MLB_LOGO_SMALL: RgbaImage =
        image::load_from_memory_with_format(MLB_LOGO_SMALL_BYTES, ImageFormat::JPEG)
            .unwrap()
            .into_rgba();
    static ref LEFT_ARROW: RgbaImage =
        image::load_from_memory_with_format(LEFT_ARROW_BYTES, ImageFormat::PNG)
            .unwrap()
            .into_rgba();
    static ref RIGHT_ARROW: RgbaImage =
        image::load_from_memory_with_format(RIGHT_ARROW_BYTES, ImageFormat::PNG)
            .unwrap()
            .into_rgba();
}

#[tokio::main]
async fn main() {
    // Well, I know the name of the org I'm interviewing with. So I've got that going for me.
    let title = "Disney Streaming Services";
    // It's kind of a useless thing to .await immediately upon application startup as it is
    // blocking the window from rendering. I stretched for having the photos load
    // asynchronously, however getting that initial API call to load in the background as well
    // would have been a bit much for such a short time frame. Backlog candidate.
    let mut schedule: Schedule = api::Schedule::try_from(api::DEFAULT).await.unwrap().into();
    // I chose piston simply because my quick experimentation with other libraries, such as glium,
    // asked me to write GLSL code and feed that into macros for consumption by OpenGL. I don't
    // vectors and shading and all that jazz, I just needed a 2D window.
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new(title, [1920, 1080])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    // We're going to be using this context repeatedly in each loop.
    // Calling something a ThingContext that takes in ThingFactory is so library specific and
    // mysterious that I admit that I do not understand the original intent here. My use of
    // this library was purely a panic to find any reasonable 2D graphics library that could see
    // me through this ordeal. So I admit that this is a case of satisfying the API without
    // any real deep understanding of what they are asking of me here.
    let mut ctx = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let fullscreen = graphics::image::Image::new().rect([0.0, 0.0, 1920.0, 1080.0]);
    let background: piston_window::G2dTexture = piston_window::Texture::from_image(
        &mut ctx,
        &(*BACKGROUND),
        &piston_window::TextureSettings::new(),
    )
    .unwrap();
    // Glyphs are the font cache that we will be using for this application.
    //
    // It's a shame, I found a cool open source font that looked very much like that blocky
    // MLB sans serif font, however it has a very anemic selection of symbols and just looked
    // back when dealing with non-alpha text.
    let mut glyphs = Glyphs::from_bytes(
        FONT,
        piston_window::TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        },
        piston_window::TextureSettings::new(),
    )
    .unwrap();
    // This is me TRYING to make this a bit more efficient. The downside of using this easy 2D
    // library is that I have apparently inherited and rather inefficient event loop
    // (see https://github.com/PistonDevelopers/piston/issues/1109). Frankly, I should NOT be
    // consuming 50MB of RAM and a nearly 1-2% of CPU, but firing up this event loop even
    // a completely blank screen will force me into that consumption, and that is unfortunate.
    //
    // However, limiting the frame rate cuts the CPU usage (on my box) down to under 1% at least.
    // This framerate seemed like a fair emulation of how quickly these sorts of menus tend
    // to render on actual TVs.
    window.set_max_fps(10);
    while let Some(e) = window.next() {
        // Move the cursor on key-up events. I would kinda like to implement fast scrolling
        // via long key holds. But alas, into the backlog it goes.
        match e.release_args() {
            Some(piston_window::Button::Keyboard(piston_window::Key::Left)) => {
                schedule.left();
            }
            Some(piston_window::Button::Keyboard(piston_window::Key::Right)) => {
                schedule.right();
            }
            _ => (),
        };
        window.draw_2d(&e, |c, g, device| {
            // This is the main rendering loop as per piston convention.
            //
            // I admit that these X/Y transformations are more of a result
            // of me experimenting around to get an orientation on the page
            // and seeing what works aesthetically. I did do some manual computations
            // to get an idea of where these objects should lay on the screen.
            // However, by and large, I am admitting that this applications is not
            // "responsive" in the sense that it does not respond to different sizes.
            // In Agile terms, I reckon that I would put that work onto the next sprint.
            piston_window::clear(BLACK, g);
            fullscreen.draw(&background, &graphics::DrawState::default(), c.transform, g);
            // The first item is padded from the left most wall of the screen.
            let mut left_edge = PADDING;
            // And the right edge is computed as the left_edge plus
            // whatever the width of the image is.
            let mut right_edge: f64;
            for item in schedule.page() {
                match item {
                    Snippet::Large(image, heading, subheading) => {
                        right_edge = left_edge + image.width() as f64;
                        let rect = graphics::image::Image::new().rect([
                            0.0,
                            0.0,
                            image.width() as f64,
                            image.height() as f64,
                        ]);
                        let txt = piston_window::Texture::from_image(
                            &mut ctx,
                            image,
                            &piston_window::TextureSettings::new(),
                        )
                        .unwrap();
                        rect.draw(
                            &txt,
                            &graphics::DrawState::default(),
                            c.transform.trans(left_edge, 540.0),
                            g,
                        );
                        // Render our header and subheader
                        piston_window::text(
                            WHITE,
                            16,
                            heading,
                            &mut glyphs,
                            c.transform.trans(left_edge + 40.0, 500.0),
                            g,
                        )
                        .unwrap();
                        piston_window::text(
                            WHITE,
                            16,
                            subheading,
                            &mut glyphs,
                            c.transform.trans(left_edge, 855.0),
                            g,
                        )
                        .unwrap();
                        glyphs.factory.encoder.flush(device);
                    }
                    Snippet::Small(image) => {
                        right_edge = left_edge + image.width() as f64;
                        let rect = graphics::image::Image::new().rect([
                            0.0,
                            0.0,
                            image.width() as f64,
                            image.height() as f64,
                        ]);
                        let txt = piston_window::Texture::from_image(
                            &mut ctx,
                            image,
                            &piston_window::TextureSettings::new(),
                        )
                        .unwrap();
                        rect.draw(
                            &txt,
                            &graphics::DrawState::default(),
                            c.transform.trans(left_edge, 578.5),
                            g,
                        );
                    }
                }
                // This is computing the small padding inbetween snippets.
                left_edge = right_edge + 27.5;
            }
            if schedule.has_less() {
                let txt = piston_window::Texture::from_image(
                    &mut ctx,
                    &*LEFT_ARROW,
                    &piston_window::TextureSettings::new(),
                ).unwrap();
                let rect = graphics::image::Image::new().rect([
                    0.0,
                    0.0,
                    LEFT_ARROW.width() as f64,
                    LEFT_ARROW.height() as f64,
                ]);
                rect.draw(&txt, &graphics::DrawState::default(), c.transform, g);
            }
            if schedule.has_more() {
                let txt = piston_window::Texture::from_image(
                    &mut ctx,
                    &*RIGHT_ARROW,
                    &piston_window::TextureSettings::new(),
                ).unwrap();
                let rect = graphics::image::Image::new().rect([
                    0.0,
                    0.0,
                    RIGHT_ARROW.width() as f64,
                    RIGHT_ARROW.height() as f64,
                ]);
                rect.draw(&txt, &graphics::DrawState::default(), c.transform.trans(1920.0 - RIGHT_ARROW.width() as f64, 0.0), g);
            }
        });
    }
}

struct Schedule {
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

enum Snippet<'a> {
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

struct Game {
    pub headline: String,
    pub subhead: String,
    pub large: Photo,
    pub small: Photo,
}

pub struct Photo {
    photo: RgbaImage,
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
            photo: image::load_from_memory_with_format(&*CANARY_BYTES, ImageFormat::JPEG)
                .unwrap()
                .into_rgba(),
            channel: rx,
        }
    }

    pub fn get(&mut self) -> Option<&RgbaImage> {
        if self.photo.len() > 4 {
            return Some(&self.photo);
        }
        match self.channel.try_recv() {
            Ok(image) => {
                self.photo = image;
                Some(&self.photo)
            }
            _ => None,
        }
    }
}
