use egui::{FontFamily, FontId};

use crate::*;

pub(super) fn render(painter: &egui::Painter, frame: &TypstFrame) {
    let origin = painter.clip_rect().left_top();
    let items = frame.items();
    render_inner(painter, origin, items);
}

fn render_item(painter: &egui::Painter, origin: Pos2, position: TypstPoint, item: &TypstFrameItem) {
    let translate_point = |point: TypstPoint| origin + vec2(to_px(point.x), to_px(point.y));
    let translate_size = |size: Size| vec2(to_px(size.x), to_px(size.y));
    let translate_stroke = |stroke: Option<TypstFixedStroke>| {
        stroke.map_or(Stroke::NONE, |stroke| {
            stroke.thickness;
            (to_px(stroke.thickness), translate_paint(&stroke.paint)).into()
        })
    };

    let position = translate_point(position);
    match item {
        TypstFrameItem::Group(TypstGroupItem {
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
        TypstFrameItem::Text(text) => {
            let font_id = match text.font {
                // ref other => todo!("font: {other:?}"),
                _ => FontId::new(
                    text.size.to_raw() as f32,
                    FontFamily::Name(Arc::from("math_font")),
                ),
            };
            // TextStyle::Body.resolve(&painter.ctx().style());
            use husky_print_utils::p;
            p!(text.text);
            p!(text.font);
            p!(position);
            p!(origin);

            painter.text(
                position,
                Align2::CENTER_CENTER,
                text.text.clone(),
                font_id,
                translate_paint(&text.fill),
            );
        }
        TypstFrameItem::Shape(
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
        TypstFrameItem::Image(..) => todo!(),
        TypstFrameItem::Meta(..) => {}
    }
}

fn render_inner<'a>(
    painter: &egui::Painter,
    origin: Pos2,
    items: impl Iterator<Item = &'a (TypstPoint, TypstFrameItem)>,
) {
    for (position, item) in items {
        render_item(painter, origin, *position, item);
    }
}
