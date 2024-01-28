use self::{input::Input, skeleton::MnistSkeleton};
use super::*;
use husky_visual_protocol::visual::shape::{Color, Point};
use ndarray::Array2;

pub struct OptimalTransport {
    source_points: Vec<Point>,
    target_points: Vec<Point>,
    optimal_transport_matrix: Array2<f64>,
}

impl OptimalTransport {
    pub(crate) fn new_ad_hoc(input: &Input, skeleton: &MnistSkeleton) -> Self {
        Self {
            source_points: vec![(2.0, 14.0).into(), (14.0, 14.0).into(), (25.0, 14.0).into()],
            target_points: vec![(14.0, 2.0).into(), (14.0, 14.0).into(), (14.0, 25.0).into()],
            optimal_transport_matrix: Array2::from_shape_fn((3, 3), |(i, j)| {
                1.0f64 / ((1 + i + j) as f64)
            }),
        }
    }
}

impl Visualize for OptimalTransport {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        let &[n, m] = self.optimal_transport_matrix.shape() else {
            unreachable!()
        };
        let mut elements: Vec<Visual> = vec![];
        for i in 0..n {
            let mut eff_weights = 0.0;
            let threshold = 0.05;
            let eff_weights: f64 = (0..m)
                .into_iter()
                .filter_map(|j| {
                    let weight = self.optimal_transport_matrix[(i, j)];
                    (weight > threshold).then_some(weight)
                })
                .sum();
            elements.extend((0..m).into_iter().filter_map(|j| {
                let weight = self.optimal_transport_matrix[(i, j)];
                (weight > threshold).then(|| -> Visual {
                    let start = self.source_points[i].into();
                    let end = self.target_points[j].into();
                    Visual::new_line_segment(
                        start,
                        end,
                        ((10.0 * weight / eff_weights) as f32, Color::Red),
                        visual_synchrotron,
                    )
                })
            }))
        }
        Visual::new_group_visual(elements, visual_synchrotron)
    }
}
