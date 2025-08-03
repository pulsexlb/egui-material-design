use egui_material::color::prelude::*;

fn main() {
    let theme = MaterialTheme::from_argb(0xffaae5a4, ColorMode::Light);
    println!("{:#?}", theme.get());
}
