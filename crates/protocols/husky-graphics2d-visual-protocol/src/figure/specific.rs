use egui::Frame;
use husky_visual_protocol::visual::primitive::PrimitiveVisual;

use super::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SpecificGraphics2dFigure {
    images: Vec<ImageVisual>,
    shapes: Vec<ShapeVisual>,
}

/// # constructor
impl SpecificGraphics2dFigure {
    pub(super) fn new(
        followed_visual: Option<(TraceId, ValReprInterface)>,
        accompanyings: &[(TraceId, ValReprInterface)],
        mut f: impl FnMut(ValReprInterface, &mut VisualSynchrotron) -> Visual,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        Self::new_aux(
            followed_visual.map(|(trace_id, val_repr_interface)| {
                (trace_id, f(val_repr_interface, visual_synchrotron))
            }),
            accompanyings
                .iter()
                .map(|&(trace_id, val_repr_interface)| {
                    (trace_id, f(val_repr_interface, visual_synchrotron))
                })
                .collect::<Vec<_>>(),
            visual_synchrotron,
        )
        .into()
    }

    fn new_aux(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: impl IntoIterator<Item = (TraceId, Visual)>,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        let mut builder = SpecificGraphics2dFigureBuilder::new(visual_synchrotron);
        builder.collect_all(followed_visual.into_iter().chain(accompanying_visuals));
        builder.finish()
    }
}

/// # builder
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

/// # ui
impl SpecificGraphics2dFigure {
    pub(super) fn specific_figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        let x = ui.available_width();
        let y = ui.available_height();
        let l = x.min(y);
        let d = (l * 0.9).floor();
        let m = (l - d) * 0.5;
        let (_, rect) = Frame::none()
            .inner_margin(m)
            .show(ui, |ui| ui.allocate_space(Vec2::splat(d)))
            .inner;
        for &image in &self.images {
            ui.painter().image(
                cache.texture_id(image, visual_synchrotron, ui),
                rect,
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
        let mnist_visual_rect = VisualRect::mnist();
        let t = |point: Point| point.to_screen(mnist_visual_rect, rect);
        for shape in &self.shapes {
            match shape.data(visual_synchrotron) {
                &ShapeVisualData::LineSegment { start, end, stroke } => {
                    ui.painter().line_segment([t(start), t(end)], stroke);
                }
                ShapeVisualData::Contour { points } => {
                    for i in 0..(points.len() - 1) {
                        ui.painter()
                            .line_segment([t(points[i]), t(points[i + 1])], (2.0, Color32::WHITE));
                    }
                }
            }
        }
    }
}
