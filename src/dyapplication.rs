use libloading::Library;
pub struct Application(pub Library);
impl Application {
    fn get_holding_style(&self) -> Card_Style {
        unsafe {
            let f = self.0.get::<fn() -> Card_Style>(b"get_holding_style\0").unwrap();
            f()
        }
    }
}