use egui::{FontData, FontDefinitions};
use std::sync::Arc;

pub(crate) fn set_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "math_font".to_owned(),
        FontData::from_static(include_bytes!(
            "../../../../assets/fonts/NewCMMath-Regular.otf"
        )),
    ); // .ttf and .otf supported

    fonts
        .families
        .entry(egui::FontFamily::Name(Arc::from("math_font")))
        .or_default()
        .insert(0, "math_font".to_owned());

    ctx.set_fonts(fonts)
}
