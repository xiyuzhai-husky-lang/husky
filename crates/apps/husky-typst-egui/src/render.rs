use crate::*;

pub(super) fn render(painter: &egui::Painter, frame: &TexFrame) {
    let origin = painter.clip_rect().left_top();
    let items = frame.items();
    render_inner(painter, origin, items);
}

fn render_item(painter: &egui::Painter, origin: Pos2, position: Point, item: &FrameItem) {
    let translate_point = |point: Point| origin + vec2(to_px(point.x), to_px(point.y));
    let translate_size = |size: Size| vec2(to_px(size.x), to_px(size.y));
    let translate_stroke = |stroke: Option<TexFixedStroke>| {
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
            use husky_print_utils::p;
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
        FrameItem::Shape(
            TexShape {
                geometry,
                fill,
                stroke,
            },
            _span,
        ) => match geometry {
            TexGeometry::Line(to_point) => {
                painter.line_segment(
                    [position, translate_point(*to_point)],
                    translate_stroke(stroke.as_ref().cloned()),
                );
            }
            TexGeometry::Rect(size) => {
                painter.rect(
                    Rect::from_min_size(position, translate_size(*size)),
                    Rounding::default(),
                    fill.as_ref().map_or(Color32::TRANSPARENT, translate_paint),
                    translate_stroke(stroke.as_ref().cloned()),
                );
            }
            TexGeometry::Path(..) => todo!(),
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
