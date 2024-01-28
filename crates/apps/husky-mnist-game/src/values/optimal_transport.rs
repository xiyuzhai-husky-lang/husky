use crate::values::skeleton::MnistBone;

use self::{alg::ot::optimal_transport_for_2d_points, input::Input, skeleton::MnistSkeleton};
use super::*;
use husky_visual_protocol::visual::shape::{Color, Point};
use ndarray::Array2;

pub struct OptimalTransport {
    source_points: Vec<Point>,
    target_points: Vec<Point>,
    optimal_transport_matrix: Array2<f64>,
}

impl OptimalTransport {
    pub(crate) fn new(input: &Input, skeleton: &MnistSkeleton) -> Self {
        let source_points = skeleton_points(skeleton);
        let target_points = input_points(input);
        let optimal_transport_matrix =
            optimal_transport_for_2d_points(&source_points, &target_points);
        Self {
            source_points,
            target_points,
            optimal_transport_matrix,
        }
    }
}

fn skeleton_points(skeleton: &MnistSkeleton) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    for bone in skeleton.bones() {
        match bone {
            MnistBone::LineSegment { start, end } => {
                let n = 10;
                for i in 0..(n + 1) {
                    let a: f32 = (i as f32) / (n as f32);
                    let b = 1.0 - a;
                    let point = ((start.x * a + end.x * b), (start.x * a + end.y * b)).into();
                    points.push(point)
                }
            }
        }
    }
    points
}

fn input_points(input: &Input) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    for i in 0..28 {
        for j in 0..28 {
            if input.pixel(i, j) {
                points.push((j as f32 - 0.5, (28 - i) as f32 - 0.5).into())
            }
        }
    }
    points
}

impl Visualize for OptimalTransport {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        let &[n, m] = self.optimal_transport_matrix.shape() else {
            unreachable!()
        };
        let mut elements: Vec<Visual> = vec![];
        for i in 0..n {
            let mut eff_weights = 0.0;
            let threshold = 0.001;
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
