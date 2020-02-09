#[macro_use]
extern crate lazy_static;
extern crate graphics;
extern crate opengles_graphics;
extern crate piston_window;
extern crate hyper;

use graphics::rectangle::square;
use image::{ImageFormat, GenericImageView, RgbaImage};
use piston_window::{EventLoop, Glyphs, Transformed, GenericEvent, ReleaseEvent, Texture, PistonWindow};

use crate::hyper::body::HttpBody;

use tokio::io::AsyncRead;
use std::process::exit;

use serde::Deserialize;

static BACKGROUND_BYTES: &[u8] = include_bytes!("../assets/background.jpg");
static MLB_LOGO_LARGE_BYTES: &[u8] = include_bytes!("../assets/mlb_logo_large.jpg");
static MLB_LOGO_SMALL_BYTES: &[u8] = include_bytes!("../assets/mlb_logo_large.jpg");
// https://www.ffonts.net/MLB-Block.font
static FONT: &[u8] = include_bytes!("../MLBBLOCK.TTF");
static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
static CENTER: [f64; 4] = [720.0, 415.0, 64.0, 36.0];
static LEFT: [f64; 4] = [720.0 - 480.0 - 160.0, 415.0, 480.0, 270.0];
static RIGHT: [f64; 4] = [720.0 + 480.0 + 160.0, 415.0, 480.0, 270.0];

static LARGE: [f64; 2] = [209.0, 118.0];
static SMALL: [f64; 2] = [135.0, 77.0];

lazy_static!(
    static ref BACKGROUND: RgbaImage = image::load_from_memory_with_format(BACKGROUND_BYTES, ImageFormat::JPEG).unwrap().into_rgba();
    static ref MLB_LOGO_LARGE: RgbaImage = image::load_from_memory_with_format(MLB_LOGO_LARGE_BYTES, ImageFormat::JPEG).unwrap().into_rgba();
    static ref MLB_LOGO_SMALL: RgbaImage = image::load_from_memory_with_format(MLB_LOGO_SMALL_BYTES, ImageFormat::JPEG).unwrap().into_rgba();
);

#[tokio::main]
async fn main() {
    let title = "Disney Streaming Services";
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new(title, [1920, 1080])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    let mut tc = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let fullscreen = graphics::image::Image::new().rect(square(0.0, 0.0, 1920.0));
    let mut large_snippets = vec![];
    for i in 0..15 {
        large_snippets.push(graphics::image::Image::new().rect(square(0.0,0.0, LARGE[0])));
    }
    let mut small_snippets = vec![];
    for i in 0..15 {
        small_snippets.push(graphics::image::Image::new().rect(square(0.0,0.0, SMALL[0])));
    }
    let small: piston_window::G2dTexture = piston_window::Texture::from_image(
        &mut piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
        },
        &(*MLB_LOGO_SMALL),
        &piston_window::TextureSettings::new(),
    ).unwrap();
    let mut current = 0;
    while let Some(e) = window.next() {
        match e.release_args() {
            Some(piston_window::Button::Keyboard(piston_window::Key::Left)) if current > 0 => current -= 1,
            Some(piston_window::Button::Keyboard(piston_window::Key::Right)) if current < 15 => current += 1,
            _ => ()
        };
        window.draw_2d(&e, |c, g, device| {
            piston_window::clear([0.0, 0.0, 0.0, 0.0], g);
            let mut left_offset = 50.0;
            let mut right_edge = left_offset + SMALL[0];
            for i in 0..15 {
                if i == current {
                    right_edge = left_offset + LARGE[0];
                    large_snippets.get(i).unwrap().draw(&small,&graphics::DrawState::default(), c.transform.trans(left_offset, 540.0), g);
                } else {
                    right_edge = left_offset + SMALL[0];
                    small_snippets.get(i).unwrap().draw(&small,&graphics::DrawState::default(), c.transform.trans(left_offset, 578.5), g);
                }
                left_offset = right_edge + 50.0;
            }
        });
    }
}

struct Image {
    inner: RgbaImage,

}

//#[tokio::main]
//async fn main() {
////    let data = get().await;
////    println!("{}", data);
////    exit(1);
//    let title = "Disney Streaming Services";
//    let mut window: piston_window::PistonWindow =
//        piston_window::WindowSettings::new(title, [1920, 1080])
//            .exit_on_esc(true)
//            .build()
//            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
//    //    window.set_lazy(true);
//    let mut tc = piston_window::TextureContext {
//        factory: window.factory.clone(),
//        encoder: window.factory.create_command_buffer().into(),
//    };
//
//    let jpg = image::load_from_memory_with_format(BACKGROUND_BYTES, ImageFormat::JPEG).unwrap();
//    let r: RgbaImage = jpg.into_rgba();
//    let imaget = graphics::image::Image::new().rect(square(0.0, 0.0, jpg.width() as f64));
//    let t: piston_window::G2dTexture = piston_window::Texture::from_image(
//        &mut tc,
//        &r,
//        &piston_window::TextureSettings::new(),
//    )
//    .unwrap();
//    let mut glyphs = Glyphs::from_bytes(FONT,tc, piston_window::TextureSettings::new()).unwrap();
//    let mut changed = true;
//    let data = vec!["a", "b", "c", "d"];
//    let mut cursor = 1;
//    let mut left = 0;
//    let mut center = 1;
//    let mut right = 2;
//    while let Some(e) = window.next() {
//        if let Some(piston_window::Button::Keyboard(piston_window::Key::Left)) = e.release_args() {
//            if cursor != 1 {
//                left = left - 1;
//                center = center - 1;
//                right = right - 1;
//                cursor -= 1;
//            }
//        }
//        if let Some(piston_window::Button::Keyboard(piston_window::Key::Right)) = e.release_args() {
//            if cursor != 2 {
//                left = left + 1;
//                center = center + 1;
//                right = right + 1;
//                cursor += 1;
//            }
//        }
//        window.draw_2d(&e, |c, g, device| {
//            imaget.draw(&t, &graphics::DrawState::default(), c.transform, g);
//            piston_window::rectangle(BLACK, CENTER, c.transform, g);
//            piston_window::rectangle(BLACK, LEFT, c.transform, g);
//            piston_window::rectangle(BLACK, RIGHT, c.transform, g);
//            piston_window::text(WHITE,32,data[center as usize], &mut glyphs, c.transform.trans(CENTER[0], 395.0), g).unwrap();
//            piston_window::text(WHITE,32,data[left as usize], &mut glyphs, c.transform.trans(LEFT[0], 395.0), g).unwrap();
//            piston_window::text(WHITE,32,data[right as usize], &mut glyphs, c.transform.trans(RIGHT[0], 395.0), g).unwrap();
//            glyphs.factory.encoder.flush(device);
//        });
//    }
//}


async fn get() -> serde_json::Value {
    let mut resp = hyper::Client::default().get("http://statsapi.mlb.com/api/v1/schedule?hydrate=game(content(editorial(recap))),decisions&date=2018-06-10&sportId=1".parse().unwrap()).await.unwrap();
    let buf = hyper::body::to_bytes(resp).await.unwrap();
    serde_json::from_slice(&buf).unwrap()
}

struct ResolvedSchedule {
    pub games: Vec<ResolvedGame>
}

struct ResolvedGame {
    pub headline: String,
    pub subhead: String,
    large: Option<piston_window::G2dTexture>,
    small: Option<piston_window::G2dTexture>
}

struct ResolvedPhoto {
    pub src: url::Url,
    photo: Option<piston_window::G2dTexture>
}

impl ResolvedPhoto {
//    pub async fn get(&mut self, window: &PistonWindow) -> &piston_window::G2dTexture {
//        match &self.photo {
//            Some(texture) => texture,
//            None => self.resolve(window).await
//        }
//    }
//
//    async fn resolve(&mut self, window: &PistonWindow) {
//
//    }
}

#[derive(Deserialize, Debug)]
struct Schedule {
    pub copyright: String,
    pub dates: Vec<Date>
}

#[derive(Deserialize, Debug)]
struct Date {
    pub date: String,
    pub games: Vec<Game>
}

#[derive(Deserialize, Debug)]
struct Game {
    pub content: Content
}

#[derive(Deserialize, Debug)]
struct Content {
    pub editorial: Editorial
}

#[derive(Deserialize, Debug)]
struct Editorial {
    pub recap: Recap
}

#[derive(Deserialize, Debug)]
struct Recap {
    pub home: Home,
}

#[derive(Deserialize, Debug)]
struct Home {
    pub headline: String,
    pub subhead: String,
    pub photo: Photos
}

#[derive(Deserialize, Debug)]
struct Photos {
    pub cuts: Cuts
}

#[derive(Deserialize, Debug)]
struct Cuts {
    #[serde(alias = "209x118")]
    pub large: Photo,
    #[serde(alias = "135x77")]
    pub small: Photo
}

#[derive(Deserialize, Debug)]
struct Photo {
    pub width: u32,
    pub height: u32,
    pub src: String,
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

