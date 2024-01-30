#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
mod alg;
mod app;
mod db;
mod op;
mod trace;
mod values;
mod view;

use self::app::*;
use self::db::*;
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual, visualize::Visualize};

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
