#[macro_use]
extern crate lazy_static;

use image::{ImageFormat, RgbaImage};
use piston_window::{EventLoop, Glyphs, ReleaseEvent, Transformed};

use serde::Deserialize;
use std::convert::TryFrom;

mod api;

static BACKGROUND_BYTES: &[u8] = include_bytes!("../assets/background.jpg");
static MLB_LOGO_LARGE_BYTES: &[u8] = include_bytes!("../assets/mlb_logo_large.jpg");
static MLB_LOGO_SMALL_BYTES: &[u8] = include_bytes!("../assets/mlb_logo_large.jpg");
static CANARY_BYTES: &[u8] = include_bytes!("../assets/canary.jpg");
static FONT: &[u8] = include_bytes!("../OpenSans-Bold.ttf");
static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
static LARGE: [f64; 2] = [480.0, 270.0];
static SMALL: [f64; 2] = [320.0, 180.0];

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
}

#[tokio::main]
async fn main() {
//    let title = "Disney Streaming Services";
//    let mut schedule: Schedule = api::Schedule::try_from(api::DEFAULT).await.unwrap().into();
//    let mut games = schedule.games;
//    let mut window: piston_window::PistonWindow =
//        piston_window::WindowSettings::new(title, [1920, 1080])
//            .exit_on_esc(true)
//            .build()
//            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
//
//    let mut ctx = piston_window::TextureContext {
//        factory: window.factory.clone(),
//        encoder: window.factory.create_command_buffer().into(),
//    };
//    let fullscreen = graphics::image::Image::new().rect([0.0, 0.0, 1920.0, 1080.0]);
//    let mut large_snippets = vec![];
//    for _ in 0..15 {
//        large_snippets.push(graphics::image::Image::new().rect([0.0, 0.0, LARGE[0], LARGE[1]]));
//    }
//    let mut small_snippets = vec![];
//    for _ in 0..15 {
//        small_snippets.push(graphics::image::Image::new().rect([0.0, 0.0, SMALL[0], SMALL[1]]));
//    }
//    let small: piston_window::G2dTexture = piston_window::Texture::from_image(
//        &mut ctx,
//        &(*MLB_LOGO_SMALL),
//        &piston_window::TextureSettings::new(),
//    )
//    .unwrap();
//    let large: piston_window::G2dTexture = piston_window::Texture::from_image(
//        &mut ctx,
//        &(*MLB_LOGO_LARGE),
//        &piston_window::TextureSettings::new(),
//    )
//    .unwrap();
//    let background: piston_window::G2dTexture = piston_window::Texture::from_image(
//        &mut ctx,
//        &(*BACKGROUND),
//        &piston_window::TextureSettings::new(),
//    )
//    .unwrap();
//
//    let mut glyphs = Glyphs::from_bytes(
//        FONT,
//        piston_window::TextureContext {
//            factory: window.factory.clone(),
//            encoder: window.factory.create_command_buffer().into(),
//        },
//        piston_window::TextureSettings::new(),
//    )
//    .unwrap();
//    let mut current = 0;
//    window.set_max_fps(10);
//    while let Some(e) = window.next() {
//        match e.release_args() {
//            Some(piston_window::Button::Keyboard(piston_window::Key::Left)) if current > 0 => {
//                current -= 1
//            }
//            Some(piston_window::Button::Keyboard(piston_window::Key::Right)) if current < 14 => {
//                current += 1
//            }
//            _ => (),
//        };
//        window.draw_2d(&e, |c, g, device| {
//            piston_window::clear(BLACK, g);
//            fullscreen.draw(&background, &graphics::DrawState::default(), c.transform, g);
//            for item in schedule.page() {
//                match item.2 {
//                    Cut::Large {photo} => {
//                        let rendered_photo = photo.get().unwrap_or(&*MLB_LOGO_LARGE);
//                        right_edge = left_offset + LARGE[0];
//                        large_snippets.get(i).unwrap().draw(
//                            &piston_window::Texture::from_image(
//                                &mut ctx, rendered_photo,
//                                &piston_window::TextureSettings::new(),
//                            )
//                            .unwrap(),
//                            &graphics::DrawState::default(),
//                            c.transform.trans(left_offset, 540.0),
//                            g,
//                        );
//                        piston_window::text(
//                            WHITE,
//                            16,
//                            &games.get(i).unwrap().headline,
//                            &mut glyphs,
//                            c.transform.trans(left_offset + 40.0, 500.0),
//                            g,
//                        )
//                        .unwrap();
//                        piston_window::text(
//                            WHITE,
//                            16,
//                            &games.get(i).unwrap().subhead,
//                            &mut glyphs,
//                            c.transform.trans(left_offset, 855.0),
//                            g,
//                        )
//                        .unwrap();
//                        glyphs.factory.encoder.flush(device);
//                    },
//                    Cut::Small {photo} => ()
//                }
//            }
//            let page = current / 5;
//            let left = page * 5;
//            let right = left + 5;
//            let mut left_offset = 27.5;
//            let mut right_edge = left_offset + SMALL[0];
//            for i in left..right {
//                if i == current {
//                    right_edge = left_offset + LARGE[0];
//                    large_snippets.get(i).unwrap().draw(
//                        &piston_window::Texture::from_image(
//                            &mut ctx,
//                            &games.get_mut(i).unwrap().large.get(),
//                            &piston_window::TextureSettings::new(),
//                        )
//                        .unwrap(),
//                        &graphics::DrawState::default(),
//                        c.transform.trans(left_offset, 540.0),
//                        g,
//                    );
//                    piston_window::text(
//                        WHITE,
//                        16,
//                        &games.get(i).unwrap().headline,
//                        &mut glyphs,
//                        c.transform.trans(left_offset + 40.0, 500.0),
//                        g,
//                    )
//                    .unwrap();
//                    piston_window::text(
//                        WHITE,
//                        16,
//                        &games.get(i).unwrap().subhead,
//                        &mut glyphs,
//                        c.transform.trans(left_offset, 855.0),
//                        g,
//                    )
//                    .unwrap();
//                    glyphs.factory.encoder.flush(device);
//                } else {
//                    right_edge = left_offset + SMALL[0];
//                    small_snippets.get(i).unwrap().draw(
//                        &piston_window::Texture::from_image(
//                            &mut ctx,
//                            &games.get_mut(i).unwrap().small.get(),
//                            &piston_window::TextureSettings::new(),
//                        )
//                        .unwrap(),
//                        &graphics::DrawState::default(),
//                        c.transform.trans(left_offset, 578.5),
//                        g,
//                    );
//                }
//                left_offset = right_edge + 27.5;
//            }
//        });
//    }
}

struct Schedule {
    pub games: Vec<Game>,
    cursor: usize,
}

impl Schedule {

    pub fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.cursor < self.games.len() - 1 {
            self.cursor += 1;
        }
    }

    pub fn page(&mut self) -> Vec<(&str, &str, &mut Cut)> {
        let page = self.cursor / 5;
        let relative_cursor = self.cursor % 5;
        let left = page * 5;
        let right = match left + 5 {
            right if right < self.games.len() - 1 => right,
            _ => self.games.len() - 1
        };
        let mut ret = vec![];
        for i in left..right {
            let game = self.games.get_mut(i).unwrap();
            let cut: &mut Cut;
            if i == relative_cursor {
                cut = &mut game.large;
            } else {
                cut = &mut game.small;
            }
            ret.push((game.headline.as_str(), game.subhead.as_str(), cut));
        }
        ret
    }
}

impl From<api::Schedule> for Schedule {
    fn from(mut schedule: api::Schedule) -> Self {
        let mut games = vec![];
        for game in schedule.dates.pop().unwrap().games.into_iter() {
            games.push(Game {
                headline: game.content.editorial.recap.home.headline.clone(),
                subhead: game.content.editorial.recap.home.subhead.clone(),
                large: Cut::Large{photo: Photo::new(
                    game.content
                        .editorial
                        .recap
                        .home
                        .photo
                        .cuts
                        .large
                        .src
                )},
                small: Cut::Small{photo: Photo::new(
                    game.content
                        .editorial
                        .recap
                        .home
                        .photo
                        .cuts
                        .small
                        .src
                )},
            });
        }
        Schedule { games, cursor: 0 }
    }
}

struct Game {
    pub headline: String,
    pub subhead: String,
    pub large: Cut,
    pub small: Cut,
}

pub enum Cut {
    Large{photo: Photo},
    Small{photo: Photo}
}

impl Cut {
//    pub fn get(&mut self) -> &RgbaImage {
//        match (self) {
//            Cut::Large{photo: photo} => match photo.get() {
//                Some(image) => image,
//                None => &*MLB_LOGO_LARGE
//            },
//            Cut::Small{photo: photo} => match photo.get() {
//                Some(image) => image,
//                None => &*MLB_LOGO_SMALL
//            },
//        }
//    }
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

trait Wrap {
    fn wrap(&self) -> String;
}

impl Wrap for String {
    fn wrap(&self) -> String {
        let mut wrapped = String::new();
        let mut length = 0;
        for byte in self.as_bytes() {
            // Avoid chopping text off in the middle of a word.
            if length > 5 && byte.is_ascii_whitespace() {
                wrapped.push('\n');
                length = 0;
            }
            wrapped.push(*byte as char);
            length += 1;
        }
        wrapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATE: &[u8] = include_bytes!("test.json");
}
