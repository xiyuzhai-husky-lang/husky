use husky_visual_protocol::visual::primitive::PrimitiveVisual;

use super::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SpecificGraphics2dFigure {
    images: Vec<ImageVisual>,
    shapes: Vec<ShapeVisual>,
}

impl SpecificGraphics2dFigure {
    pub(super) fn new(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: impl Iterator<Item = (TraceId, Visual)>,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        let mut builder = SpecificGraphics2dFigureBuilder::new(visual_synchrotron);
        builder.collect_all(followed_visual.into_iter().chain(accompanying_visuals));
        builder.finish()
    }

    pub(super) fn figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        let x = ui.available_width() * 0.8;
        let y = ui.available_height() / 2.;
        let d = x.min(y);
        let (_, rect) = ui.allocate_space(Vec2::splat(d));
        for &image in &self.images {
            ui.painter().image(
                cache.texture_id(image, visual_synchrotron, ui),
                rect,
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            )
        }
        let mnist_visual_rect = VisualRect::mnist();
        let t = |point: Point| point.to_screen(mnist_visual_rect, rect);
        for shape in &self.shapes {
            use husky_print_utils::p;
            match shape.data(visual_synchrotron) {
                &ShapeVisualData::LineSegment { start, end } => ui
                    .painter()
                    .line_segment([t(start), t(end)], (2.0, Color32::YELLOW)),
                ShapeVisualData::Contour { points } => {
                    for i in 0..(points.len() - 1) {
                        ui.painter()
                            .line_segment([t(points[i]), t(points[i + 1])], (2.0, Color32::WHITE))
                    }
                }
            }
        }
    }
}

struct SpecificGraphics2dFigureBuilder<'a> {
    images: Vec<ImageVisual>,
    shapes: Vec<ShapeVisual>,
    primitives: Vec<(TraceId, PrimitiveVisual)>,
    visual_synchrotron: &'a VisualSynchrotron,
}

impl<'a> SpecificGraphics2dFigureBuilder<'a> {
    fn new(visual_synchrotron: &'a VisualSynchrotron) -> Self {
        Self {
            images: vec![],
            shapes: vec![],
            primitives: vec![],
            visual_synchrotron,
        }
    }

    fn collect_all(&mut self, visuals: impl Iterator<Item = (TraceId, Visual)>) {
        visuals.for_each(|(trace_id, visual)| self.collect(trace_id, visual))
    }

    fn collect(&mut self, trace_id: TraceId, visual: Visual) {
        match visual {
            Visual::Void => (),
            Visual::Primitive(primitive) => self.primitives.push((trace_id, primitive)),
            Visual::Text(_) => todo!(),
            Visual::RichText(_) => todo!(),
            Visual::Image(image) => self.images.push(image),
            Visual::Shape(shape) => self.shapes.push(shape),
            Visual::Mesh(_) => todo!(),
            Visual::Video(_) => todo!(),
            Visual::Group(group) => {
                for &element in group.elements(self.visual_synchrotron) {
                    self.collect(trace_id, element)
                }
            }
        }
    }

    fn finish(self) -> SpecificGraphics2dFigure {
        SpecificGraphics2dFigure {
            images: self.images,
            shapes: self.shapes,
        }
    }
}

impl Graphics2dFigure {
    fn new_specific_aux() -> Self {
        todo!()
    }
}
