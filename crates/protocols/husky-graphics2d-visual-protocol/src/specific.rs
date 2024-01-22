use husky_visual_protocol::visual::primitive::PrimitiveVisual;

use super::*;

pub(super) fn new_specific_figure(
    followed_visual: Option<(TraceId, Visual)>,
    accompanying_visuals: impl Iterator<Item = (TraceId, Visual)>,
    visual_synchrotron: &VisualSynchrotron,
) -> Graphics2dFigure {
    let mut builder = Graphics2dFigureBuilder::new(visual_synchrotron);
    builder.collect_all(followed_visual.into_iter().chain(accompanying_visuals));
    builder.finish()
}

struct Graphics2dFigureBuilder<'a> {
    images: Vec<ImageVisual>,
    shapes: Vec<ShapeVisual>,
    primitives: Vec<(TraceId, PrimitiveVisual)>,
    visual_synchrotron: &'a VisualSynchrotron,
}

impl<'a> Graphics2dFigureBuilder<'a> {
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

    fn finish(self) -> Graphics2dFigure {
        Graphics2dFigure {
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
