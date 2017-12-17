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
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Decorate {
    BACKGROUND,
    TITLE,
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
    PAGE1F,
    PAGE2F,
    PAGE3F,
    PAGE4F,
    CARDS1,
    CARDS2,
    CARDS3,
    CARDS4,
    CARDS5,
    CARDS6,
    CARDS7,
    CARDS8,
    CARDS9,
    CARDS10,
    CARDS11,
    CARDS12,
    CARDS13,
    CARDS14,
    CARDS15,
    CARDS16,
    CARDS17,
    CARDS18,
    CARDS19,
    CARDS20,
    CARDS21,
    CARDS22,
    CARDS23,
    CARDS24,
    CARDS25,
    CARDS26,
    CARDS27,
    BACKCARD,
    GAMEICONS,
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum ResourceEnum {
    Font(Font),
    Blowup(i32),
    Decorate(Decorate),
    Sprite(Sprite),
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
pub fn get_button_path() -> &'static str {
    "images/Flat Buttons.png"
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
