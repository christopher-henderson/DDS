#[macro_use]
extern crate lazy_static;
extern crate graphics;
extern crate hyper;
extern crate opengles_graphics;
extern crate piston_window;

use image::{ImageFormat, RgbaImage};
use piston_window::{
    EventLoop, GenericEvent, Glyphs, PistonWindow, ReleaseEvent, Texture, Transformed,
};

use crate::hyper::body::HttpBody;

use std::process::exit;
use tokio::io::AsyncRead;

use serde::Deserialize;
use std::collections::hash_set::SymmetricDifference;
use std::ops::Deref;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::marker::PhantomData;
use std::time::Duration;

static BACKGROUND_BYTES: &[u8] = include_bytes!("../assets/background.jpg");
static MLB_LOGO_LARGE_BYTES: &[u8] = include_bytes!("../assets/mlb_logo_large.jpg");
static MLB_LOGO_SMALL_BYTES: &[u8] = include_bytes!("../assets/mlb_logo_large.jpg");
static CANARY_BYTES: &[u8] = include_bytes!("../assets/canary.jpg");
// https://www.ffonts.net/MLB-Block.font
static FONT: &[u8] = include_bytes!("../OpenSans-Bold.ttf");
static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
static LARGE: [f64; 2] = [480.0, 270.0];
static SMALL: [f64; 2] = [320.0, 180.0];

use piston_window::Window;

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
    let title = "Disney Streaming Services";
    let mut games = from(get().await);
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new(title, [1920, 1080])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    let mut tc = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let fullscreen = graphics::image::Image::new().rect([0.0, 0.0, 1920.0, 1080.0]);
    let mut large_snippets = vec![];
    for i in 0..15 {
        large_snippets.push(graphics::image::Image::new().rect([0.0, 0.0, LARGE[0], LARGE[1]]));
    }
    let mut small_snippets = vec![];
    for i in 0..15 {
        small_snippets.push(graphics::image::Image::new().rect([0.0, 0.0, SMALL[0], SMALL[1]]));
    }
    let small: piston_window::G2dTexture = piston_window::Texture::from_image(
        &mut piston_window::TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        },
        &(*MLB_LOGO_SMALL),
        &piston_window::TextureSettings::new(),
    )
    .unwrap();
    let large: piston_window::G2dTexture = piston_window::Texture::from_image(
        &mut piston_window::TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        },
        &(*MLB_LOGO_LARGE),
        &piston_window::TextureSettings::new(),
    )
    .unwrap();
    let background: piston_window::G2dTexture = piston_window::Texture::from_image(
        &mut piston_window::TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        },
        &(*BACKGROUND),
        &piston_window::TextureSettings::new(),
    )
    .unwrap();
    let mut ctx = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut glyphs = Glyphs::from_bytes(
        FONT,
        piston_window::TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        },
        piston_window::TextureSettings::new(),
    )
    .unwrap();
    let mut current = 0;
    window.set_max_fps(10);
    while let Some(e) = window.next() {
        match e.release_args() {
            Some(piston_window::Button::Keyboard(piston_window::Key::Left)) if current > 0 => {
                current -= 1
            }
            Some(piston_window::Button::Keyboard(piston_window::Key::Right)) if current < 14 => {
                current += 1
            }
            _ => (),
        };
        window.draw_2d(&e, |c, g, device| {
            piston_window::clear(BLACK, g);
            fullscreen.draw(&background, &graphics::DrawState::default(), c.transform, g);
            let page = current / 5;
            let left = page * 5;
            let right = left + 5;
            let mut left_offset = 27.5;
            let mut right_edge = left_offset + SMALL[0];
            for i in left..right {
                if i == current {
                    right_edge = left_offset + LARGE[0];
                    large_snippets.get(i).unwrap().draw(
                        &piston_window::Texture::from_image(
                            &mut ctx,
                            &games.get_mut(i).unwrap().large.get(),
                            &piston_window::TextureSettings::new(),
                        )
                        .unwrap(),
                        &graphics::DrawState::default(),
                        c.transform.trans(left_offset, 540.0),
                        g,
                    );
                    piston_window::text(
                        WHITE,
                        16,
                        &games.get(i).unwrap().headline,
                        &mut glyphs,
                        c.transform.trans(left_offset + 40.0, 500.0),
                        g,
                    )
                    .unwrap();
                    piston_window::text(
                        WHITE,
                        16,
                        &games.get(i).unwrap().subhead,
                        &mut glyphs,
                        c.transform.trans(left_offset, 855.0),
                        g,
                    )
                    .unwrap();
                    glyphs.factory.encoder.flush(device);
                } else {
                    right_edge = left_offset + SMALL[0];
                    small_snippets.get(i).unwrap().draw(
                        &piston_window::Texture::from_image(
                            &mut ctx,
                            &games.get_mut(i).unwrap().small.get(),
                            &piston_window::TextureSettings::new(),
                        )
                        .unwrap(),
                        &graphics::DrawState::default(),
                        c.transform.trans(left_offset, 578.5),
                        g,
                    );
                }
                left_offset = right_edge + 27.5;
            }
        });
    }
}

struct Image {
    inner: RgbaImage,
}

async fn get() -> Schedule {
    let resp = hyper::Client::default().get("http://statsapi.mlb.com/api/v1/schedule?hydrate=game(content(editorial(recap))),decisions&date=2018-06-10&sportId=1".parse().unwrap()).await.unwrap();
    let buf = hyper::body::to_bytes(resp).await.unwrap();
    serde_json::from_slice(&buf).unwrap()
}

struct ResolvedSchedule {
    pub games: Vec<ResolvedGame>,
}

fn from(schedule: Schedule) -> Vec<ResolvedGame> {
    let mut games = vec![];
    for game in schedule.dates[0].games.iter() {
        games.push(ResolvedGame {
            headline: game.content.editorial.recap.home.headline.clone(),
            subhead: game.content.editorial.recap.home.subhead.clone(),
            large: ResolvedPhoto::new(
                game.content
                    .editorial
                    .recap
                    .home
                    .photo
                    .cuts
                    .large
                    .src
                    .clone(), &*MLB_LOGO_LARGE
            ),
            small: ResolvedPhoto::new(
                game.content
                    .editorial
                    .recap
                    .home
                    .photo
                    .cuts
                    .small
                    .src
                    .clone(), &*MLB_LOGO_SMALL
            ),
        });
    }
    games
}

struct ResolvedGame {
    pub headline: String,
    pub subhead: String,
    large: ResolvedPhoto,
    small: ResolvedPhoto,
}

struct ResolvedPhoto {
    photo: RgbaImage,
    default: &'static RgbaImage,
    channel: crossbeam_channel::Receiver<RgbaImage>
}

impl ResolvedPhoto {
    pub fn new(src: String, default: &'static RgbaImage) -> ResolvedPhoto {
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
        ResolvedPhoto { photo: image::load_from_memory_with_format(&*CANARY_BYTES, ImageFormat::JPEG)
                .unwrap()
                .into_rgba(), default, channel: rx }
    }

    pub fn get(&mut self) -> &RgbaImage {
        if self.photo.len() > 4 {
            return &self.photo;
        }
        match self.channel.try_recv() {
            Ok(image) => {
                self.photo = image;
                &self.photo
            }
            Err(crossbeam_channel::TryRecvError::Empty) => {
                self.default
            },
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                self.default
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct Schedule {
    pub copyright: String,
    pub dates: Vec<Date>,
}

#[derive(Deserialize, Debug)]
struct Date {
    pub date: String,
    pub games: Vec<Game>,
}

#[derive(Deserialize, Debug)]
struct Game {
    pub content: Content,
}

#[derive(Deserialize, Debug)]
struct Content {
    pub editorial: Editorial,
}

#[derive(Deserialize, Debug)]
struct Editorial {
    pub recap: Recap,
}

#[derive(Deserialize, Debug)]
struct Recap {
    pub home: Home,
}

#[derive(Deserialize, Debug)]
struct Home {
    pub headline: String,
    pub subhead: String,
    pub photo: Photos,
}

#[derive(Deserialize, Debug)]
struct Photos {
    pub cuts: Cuts,
}

#[derive(Deserialize, Debug)]
struct Cuts {
    #[serde(alias = "480x270")]
    pub large: Photo,
    #[serde(alias = "320x180")]
    pub small: Photo,
}

#[derive(Deserialize, Debug)]
struct Photo {
    pub width: u32,
    pub height: u32,
    pub src: String,
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

    #[test]
    fn asdsdfd() {
        let schedule: Schedule = serde_json::from_slice(TEST_DATE).unwrap();
        //        for game in schedule.dates.get(0).unwrap().games.iter() {
        //            match game.content.headline {
        //                Some(_) => (),
        //                None => println!("{:?}", game)
        //            }
        //        }
    }
}
