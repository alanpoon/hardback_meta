use std::collections::HashMap;
use cards;
use local;
#[derive(Debug)]
pub struct AppData {
    pub width: u32,
    pub height: u32,
    pub guess: String,
    pub title: String,
    pub info: String,
    pub listcards: HashMap<i32, cards::ListCard>,
    pub blowupcards: HashMap<i32, cards::BlowupCard>,
    pub texts: local::Local,
}

impl AppData {
    pub fn new(width: u32, height: u32, title: &str) -> AppData {
        let (l, b) = cards::populate();
        AppData {
            width: width,
            height: height,
            guess: String::new(),
            title: title.to_owned(),
            info: "? X".to_owned(),
            listcards: l,
            blowupcards: b,
            texts: local::Local::new(),
        }
    }
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Font {
    REGULAR,
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
    MAIN,
    FRAME,
    BOXCLOSURE,
    BROWNPAPER,
    TRANSPARENT,
    PAGEHARDCOVER,
    PAGEINTRO1,
    PAGEINTRO2,
    PAGECONTENT,
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
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum ResourceEnum {
    Giveable(cards::Giveable),
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
