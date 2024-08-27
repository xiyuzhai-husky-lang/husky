use crate::chart::StandardChartDim0;

use super::*;
use egui::Frame;
use husky_control_flow_utils::pass;
use husky_linket_impl::pedestal::JointPedestal;
use husky_standard_linket_impl::pedestal::StandardJointPedestal;
use husky_visual_protocol::visual::primitive::PrimitiveVisual;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct StandardFigureDim0 {
    joint_pedestal: StandardJointPedestal,
    plots: Vec<StandardPlot>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum StandardPlot {
    Graphics2D {
        images: Vec<ImageVisual>,
        shapes: Vec<ShapeVisual>,
    },
    Graphics3D {
        meshes: (),
    },
    Text,
    Code,
}

/// # constructor
impl StandardFigureDim0 {
    pub(super) fn from_chart(
        (joint_pedestal, composite_visual): StandardChartDim0<CompositeVisual<TraceId>>,
        trace_plot_map: &[(TraceId, usize)],
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        Self::new(
            joint_pedestal,
            composite_visual
                .followed_reduced
                .into_iter()
                .chain(composite_visual.accompanyings_except_followed_reduced),
            trace_plot_map,
            visual_synchrotron,
        )
    }

    fn new(
        joint_pedestal: StandardJointPedestal,
        traced_visuals: impl IntoIterator<Item = (TraceId, Visual)>,
        trace_plot_map: &[(TraceId, usize)],
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        let mut builder =
            StandardFigureBuilder::new(joint_pedestal, trace_plot_map, visual_synchrotron);
        builder.collect_all(traced_visuals.into_iter());
        builder.finish()
    }
}

/// # builder
struct StandardFigureBuilder<'a> {
    joint_pedestal: StandardJointPedestal,
    trace_plot_map: &'a [(TraceId, usize)],
    visual_synchrotron: &'a VisualSynchrotron,
    plots: Vec<StandardPlot>,
}

impl<'a> StandardFigureBuilder<'a> {
    fn new(
        joint_pedestal: StandardJointPedestal,
        trace_plot_map: &'a [(TraceId, usize)],
        visual_synchrotron: &'a VisualSynchrotron,
    ) -> Self {
        Self {
            joint_pedestal,
            trace_plot_map,
            visual_synchrotron,
            plots: vec![],
        }
    }

    fn collect_all(&mut self, visuals: impl Iterator<Item = (TraceId, Visual)>) {
        visuals.for_each(|(trace_id, visual)| self.collect(trace_id, visual))
    }

    fn collect(&mut self, trace_id: TraceId, visual: Visual) {
        todo!()
        // match visual {
        //     Visual::Void => (),
        //     Visual::Primitive(primitive) => self.primitives.push((trace_id, primitive)),
        //     Visual::Text(_) => todo!(),
        //     Visual::RichText(_) => todo!(),
        //     Visual::Image(image) => self.images.push(image),
        //     Visual::Shape(shape) => self.shapes.push(shape),
        //     Visual::Mesh(_) => todo!(),
        //     Visual::Video(_) => todo!(),
        //     Visual::Group(group) => {
        //         for &element in group.elements(self.visual_synchrotron) {
        //             self.collect(trace_id, element)
        //         }
        //     }
        //     Visual::Math(_) => pass!("math is not displayed"),
        // }
    }

    fn finish(self) -> StandardFigureDim0 {
        StandardFigureDim0 {
            joint_pedestal: self.joint_pedestal,
            plots: self.plots,
        }
    }
}

/// # ui

impl StandardFigureDim0 {
    pub(super) fn ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        todo!()
    }
}

impl StandardPlot {
    pub(super) fn ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        match self {
            StandardPlot::Graphics2D { images, shapes } => {
                let x = ui.available_width();
                let y = ui.available_height();
                let l = x.min(y);
                let d = (l * 0.9).floor();
                let m = (l - d) * 0.5;
                let (_, rect) = Frame::none()
                    .inner_margin(m)
                    .show(ui, |ui| ui.allocate_space(Vec2::splat(d)))
                    .inner;
                for &image in images {
                    ui.painter().image(
                        cache.texture_id(image, visual_synchrotron, ui),
                        rect,
                        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
                let mnist_visual_rect = VisualRect::mnist();
                let t = |point: Point| point.to_screen(mnist_visual_rect, rect);
                for shape in shapes {
                    match shape.data(visual_synchrotron) {
                        &ShapeVisualData::LineSegment { start, end, stroke } => {
                            ui.painter().line_segment([t(start), t(end)], stroke);
                        }
                        ShapeVisualData::Contour { points } => {
                            for i in 0..(points.len() - 1) {
                                ui.painter().line_segment(
                                    [t(points[i]), t(points[i + 1])],
                                    (2.0, Color32::WHITE),
                                );
                            }
                        }
                    }
                }
            }
            StandardPlot::Graphics3D { meshes } => todo!(),
            StandardPlot::Text => todo!(),
            StandardPlot::Code => todo!(),
        }
    }
}
