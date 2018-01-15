use std::collections::HashMap;
use cards;
use local;
#[derive(Debug)]
pub struct AppData {
    pub ref_win_dim: [f64; 2],
    pub act_win_dim: [f64; 2],
    pub guess: String,
    pub title: String,
    pub info: String,
    pub blowupcards: HashMap<usize, cards::BlowupCard>,
    pub texts: local::Local,
}

impl AppData {
    pub fn new(width: f64, height: f64, title: &str) -> AppData {
        let b = cards::populate();
        AppData {
            ref_win_dim: [1024.0, 704.0],
            act_win_dim: [width, height],
            guess: String::new(),
            title: title.to_owned(),
            info: "? X".to_owned(),
            blowupcards: b,
            texts: local::Local::new(),
        }
    }
    pub fn convert_w(&self, w: f64) -> f64 {
        (w / self.ref_win_dim[0]) * self.act_win_dim[0]
    }
    pub fn convert_h(&self, h: f64) -> f64 {
        (h / self.ref_win_dim[1]) * self.act_win_dim[1]
    }
    pub fn convert_dim(&self, dim: [f64; 2]) -> [f64; 2] {
        [self.convert_w(dim[0]), self.convert_h(dim[1])]
    }
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Font {
    REGULAR,
    BOLD,
    ITALIC,
    BOLDITALIC,
    BEON,
    MYSTERY,
    HORROR,
    ADVENTURE,
    ROMANCE,
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Decorate {
    BACKGROUND,
    TITLE,
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Texture {
    PAGE1F,
    PAGE2F,
    PAGE3F,
    PAGE4F,
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Sprite {
    PAGE2,
    ICONS,
    BACKGROUND,
    RUST,
    KEYPAD,
    BUTTON,
    ARROWS,
    MAIN,
    FRAME,
    BOXCLOSURE,
    BROWNPAPER,
    TRANSPARENT,
    PAGEHARDCOVER,
    PAGEINTRO1,
    PAGEINTRO2,
    PAGECONTENT,
    DOWNLOAD,
    BACKCARD,
    GAMEICONS,
    CLOUDY,
    COININFO,
    COININFO270,
    UNOFFICIAL,
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum MusicEnum {
    BACKGROUND,
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum ChunkEnum {
    PAGEFLIP,
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum ResourceEnum {
    Font(Font),
    Blowup(i32),
    Decorate(Decorate),
    Sprite(Sprite),
    Texture(Texture),
    Music(MusicEnum),
    Chunk(ChunkEnum),
}

#[cfg(feature = "english")]
pub fn get_cards_path() -> &'static str {
    "images/english.jpg"
}
#[cfg(feature = "english")]
pub fn get_page2_path() -> &'static str {
    "images/english2.jpg"
}
#[cfg(feature = "english")]
pub fn get_icons_path() -> &'static str {
    "images/newicon.png"
}

pub fn get_background_path() -> &'static str {
    "images/starMG.png"
}
pub fn get_rust_path() -> &'static str {
    "images/rust.png"
}

pub fn get_main_path() -> &'static str {
    "images/main.jpg"
}
pub fn get_frame_path() -> &'static str {
    "images/frame.png"
}
pub fn get_boxclosure_path() -> &'static str {
    "images/boxclosure.jpg"
}
pub fn get_brownpaper_path() -> &'static str {
    "images/brownpaper.png"
}
pub fn get_transparent_path() -> &'static str {
    "images/transparent.png"
}
