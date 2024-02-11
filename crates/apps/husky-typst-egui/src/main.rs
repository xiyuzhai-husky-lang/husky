use eframe::egui;
use egui::{vec2, Align2, Color32, Pos2, Rect, Rounding, Sense, Stroke, TextStyle};
use husky_typst::{
    eval::Tracer,
    layout::Abs,
    visualize::{TexColor, TexFixedStroke, TexGeometry, TexPaint, TexRgb, TexRgba},
};
use husky_typst::{
    layout::{Frame, FrameItem, GroupItem, Point, Size},
    visualize::TexShape,
};
use render::render;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::Arc;

mod compile;
mod render;
mod sandbox;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap();
}

struct App {
    renderer: SyncSender<String>,
    rendered: Receiver<Result<Frame, String>>,

    buffer: String,
    current_frame: Option<Result<Frame, String>>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let sandbox = Arc::new(crate::sandbox::Sandbox::new());
        let mut tracer = Tracer::new();
        let compiled = husky_typst::compile(
            &Arc::clone(&sandbox).with_source("x + 1".to_string()),
            &mut tracer,
        )
        .map_err(|errors| format!("{errors:#?}"))
        .and_then(|doc| {
            doc.pages
                .into_iter()
                .next()
                .ok_or_else(|| r#"error"#.into())
        });
        let (renderer, rendered) = compile::spawn_compilation_thread(cc.egui_ctx.clone());
        println!("compiled = {:?}", compiled);
        Self {
            renderer,
            rendered,
            buffer: String::new(),
            current_frame: Some(compiled.map(|page| page.frame)),
        }
    }
}

const PIXELS_PER_POINT: f32 = 1.25;

fn to_px(abs: Abs) -> f32 {
    abs.to_pt() as f32 * PIXELS_PER_POINT
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(rendered) = self.rendered.try_recv() {
                self.current_frame = Some(rendered);
            }

            ui.text_edit_multiline(&mut self.buffer);

            if ui.button("Render").clicked() {
                self.renderer.send(self.buffer.clone()).unwrap();
            }

            match &self.current_frame {
                None => {
                    ui.label("please render");
                }
                Some(Err(error)) => {
                    ui.label(format!("Errors:\n{error}"));
                }
                Some(Ok(frame)) => {
                    egui::Frame::canvas(&ctx.style())
                        .fill(Color32::WHITE)
                        .show(ui, |ui| {
                            let (rect, _response) =
                                ui.allocate_exact_size(ui.available_size(), Sense::hover());
                            let painter = ui.painter().with_clip_rect(rect);
                            render(&painter, frame);
                        });
                }
            }
        });
    }
}

fn translate_paint(paint: &TexPaint) -> Color32 {
    match paint {
        TexPaint::Solid(color) => {
            let TexColor::Rgba(TexRgba {
                color: TexRgb {
                    red, green, blue, ..
                },
                alpha,
            }) = color.to_rgba()
            else {
                unreachable!()
            };
            Color32::from_rgba_unmultiplied(
                linear_to_discrete(red),
                linear_to_discrete(green),
                linear_to_discrete(blue),
                linear_to_discrete(alpha),
            )
        }
        TexPaint::Gradient(_) => todo!(),
        TexPaint::Pattern(_) => todo!(),
    }
}

fn linear_to_discrete(red: f32) -> u8 {
    (red * 255.0).round() as u8
}
