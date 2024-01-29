use self::{
    alg::ot::optimal_transport_for_2d_points,
    input::Input,
    skeleton::{MnistBone, MnistSkeleton},
};
use super::*;
use crate::trace::optimal_transport::OptimalTransport;
use husky_visual_protocol::visual::shape::{Color, Point};
use ndarray::Array2;

pub struct OptimalTransportAverage {
    source_points: Vec<Point>,
    average_target_points: Vec<Point>,
}

impl OptimalTransportAverage {
    pub(crate) fn new(optimal_transport: &OptimalTransport) -> Self {
        let source_points = optimal_transport.source_points().to_vec();
        let target_points = optimal_transport.target_points();
        let optimal_transport_matrix = optimal_transport.optimal_transport_matrix();
        let average_target_points = (0..source_points.len())
            .map(|i| {
                let mut average_target_point = (0., 0.);
                let mut total_weights = 0.0;
                for (j, tgt) in target_points.iter().enumerate() {
                    let weight = optimal_transport_matrix[(i, j)] as f32;
                    average_target_point.0 += weight * tgt.x();
                    average_target_point.1 += weight * tgt.y();
                    total_weights += weight;
                }
                // todo: why total_weights is not equal to one?
                average_target_point.0 /= total_weights;
                average_target_point.1 /= total_weights;
                average_target_point.into()
            })
            .collect();
        Self {
            source_points,
            average_target_points,
        }
    }
}

impl Visualize for OptimalTransportAverage {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        assert_eq!(self.source_points.len(), self.average_target_points.len());
        let n = self.source_points.len();
        let mut elements: Vec<Visual> = (0..n)
            .into_iter()
            .map(|i| {
                let start = self.source_points[i].into();
                let end = self.average_target_points[i].into();
                Visual::new_line_segment(start, end, (3.0, Color::Red), visual_synchrotron)
            })
            .collect();
        Visual::new_group_visual(elements, visual_synchrotron)
    }
}
