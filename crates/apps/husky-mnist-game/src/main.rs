#![feature(generic_const_exprs)]
mod app;
mod db;
mod op;
mod trace;
mod values;
mod view;

use self::app::*;
use self::db::*;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_line_segment_mut;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Husky Mnist Game",
        options,
        Box::new(|_cc| Box::new(MnistApp::default())),
    )
}
