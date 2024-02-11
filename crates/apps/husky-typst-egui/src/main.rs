use eframe::egui;
use egui::{vec2, Align2, Color32, Pos2, Rect, Rounding, Sense, Stroke, TextStyle};
use husky_typst::{
    eval::Tracer,
    layout::Abs,
    visualize::{TypstColor, TypstFixedStroke, TypstGeometry, TypstPaint, TypstRgb, TypstRgba},
};
use husky_typst::{
    layout::{Frame, FrameItem, GroupItem, Point, Size},
    visualize::TypstShape,
};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::Arc;

mod renderer;
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
        let (renderer, rendered) = renderer::spawn(cc.egui_ctx.clone());
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

fn translate_paint(paint: &TypstPaint) -> Color32 {
    match paint {
        TypstPaint::Solid(color) => {
            let TypstColor::Rgba(TypstRgba {
                color: TypstRgb {
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
        TypstPaint::Gradient(_) => todo!(),
        TypstPaint::Pattern(_) => todo!(),
    }
}

fn linear_to_discrete(red: f32) -> u8 {
    (red * 255.0).round() as u8
}

fn render_item(painter: &egui::Painter, origin: Pos2, position: Point, item: &FrameItem) {
    let translate_point = |point: Point| origin + vec2(to_px(point.x), to_px(point.y));
    let translate_size = |size: Size| vec2(to_px(size.x), to_px(size.y));
    let translate_stroke = |stroke: Option<TypstFixedStroke>| {
        stroke.map_or(Stroke::NONE, |stroke| {
            stroke.thickness;
            (to_px(stroke.thickness), translate_paint(&stroke.paint)).into()
        })
    };

    let position = translate_point(position);
    match item {
        FrameItem::Group(GroupItem {
            frame,
            transform,
            clip_path,
        }) => {
            assert!(
                transform.is_identity(),
                "non-identity transforms not yet implemented"
            );
            let inner_painter = if let Some(clip_path) = clip_path {
                painter.with_clip_rect(Rect::from_min_size(position, translate_size(frame.size())))
            } else {
                painter.clone()
            };
            render(&inner_painter, frame);
        }
        FrameItem::Text(text) => {
            let font_id = TextStyle::Body.resolve(&painter.ctx().style());
            painter.text(
                origin,
                Align2::CENTER_CENTER,
                text.text.clone(),
                font_id,
                translate_paint(&text.fill),
            );
        }
        FrameItem::Shape(
            TypstShape {
                geometry,
                fill,
                stroke,
            },
            _span,
        ) => match geometry {
            TypstGeometry::Line(to_point) => {
                painter.line_segment(
                    [position, translate_point(*to_point)],
                    translate_stroke(stroke.as_ref().cloned()),
                );
            }
            TypstGeometry::Rect(size) => {
                painter.rect(
                    Rect::from_min_size(position, translate_size(*size)),
                    Rounding::default(),
                    fill.as_ref().map_or(Color32::TRANSPARENT, translate_paint),
                    translate_stroke(stroke.as_ref().cloned()),
                );
            }
            TypstGeometry::Path(..) => todo!(),
        },
        FrameItem::Image(..) => todo!(),
        FrameItem::Meta(..) => {}
    }
}

fn render_inner<'a>(
    painter: &egui::Painter,
    origin: Pos2,
    items: impl Iterator<Item = &'a (Point, FrameItem)>,
) {
    for (position, item) in items {
        render_item(painter, origin, *position, item);
    }
}

fn render(painter: &egui::Painter, frame: &Frame) {
    let origin = painter.clip_rect().left_top();
    let items = frame.items();
    render_inner(painter, origin, items);
}
