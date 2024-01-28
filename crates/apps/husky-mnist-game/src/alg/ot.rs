use husky_visual_protocol::visual::shape::Point;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use ot::prelude::*;
use rust_optimal_transport as ot; // max()

pub fn optimal_transport_for_2d_points(
    source: impl IntoIterator<Item = Point>,
    target: impl IntoIterator<Item = Point>,
) -> Array2<f64> {
    use husky_print_utils::p;
    let source = points_to_array2(source);
    let mut source_weights = weights(&source);
    let target = points_to_array2(target);
    let mut target_weights = weights(&target);

    // Compute the cost between the distributions
    let mut cost = dist(&source, &target, SqEuclidean);

    // Normalize cost matrix for numerical stability
    let max_cost = cost.max().unwrap();
    cost = &cost / *max_cost;
    EarthMovers::new(&mut source_weights, &mut target_weights, &mut cost)
        .solve()
        .unwrap()
}

#[test]
fn optimal_transport_for_2d_points_works() {
    use expect_test::expect;
    expect![[r#"
        [[1.0]], shape=[1, 1], strides=[1, 1], layout=CFcf (0xf), const ndim=2
    "#]]
    .assert_debug_eq(&optimal_transport_for_2d_points(
        [Point::new(0.0, 0.0)],
        [Point::new(1.0, 0.0)],
    ));
    expect![[r#"
        [[0.5, 0.0],
         [0.0, 0.5]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2
    "#]]
    .assert_debug_eq(&optimal_transport_for_2d_points(
        [Point::new(0.0, 0.0), Point::new(1.0, 0.0)],
        [Point::new(0.0, 0.0), Point::new(1.0, 0.0)],
    ))
}

fn points_to_array2(points: impl IntoIterator<Item = Point>) -> Array2<f64> {
    let points: Vec<Point> = points.into_iter().collect();
    let n = points.len();
    Array2::<f64>::from_shape_fn((n, 2), |(i, j)| match j {
        0 => points[i].x.into_inner() as f64,
        1 => points[i].y.into_inner() as f64,
        _ => unreachable!(),
    })
}

fn weights(points: &Array2<f64>) -> Array1<f64> {
    let n = points.shape()[0];
    Array1::<f64>::from_elem(n as usize, 1. / (n as f64))
}

#[test]
fn ot_works() {
    // Generate data by sampling a 2D gaussian distribution
    let n = 1000;

    let mu_source = array![0., 0.];
    let cov_source = array![[1., 0.], [0., 1.]];

    let mu_target = array![4., 4.];
    let cov_target = array![[1., -0.8], [-0.8, 1.]];

    let source = ot::utils::sample_2D_gauss(n, &mu_source, &cov_source).unwrap();
    let target = ot::utils::sample_2D_gauss(n, &mu_target, &cov_target).unwrap();

    // Uniform weights on the source and target distributions
    let mut source_weights = Array1::<f64>::from_elem(n as usize, 1. / (n as f64));
    let mut target_weights = Array1::<f64>::from_elem(n as usize, 1. / (n as f64));

    // Compute the cost between the distributions
    let mut cost = dist(&source, &target, SqEuclidean);

    // Normalize cost matrix for numerical stability
    let max_cost = cost.max().unwrap();
    cost = &cost / *max_cost;

    // Compute the optimal transport matrix
    let ot_matrix = EarthMovers::new(&mut source_weights, &mut target_weights, &mut cost)
        .solve()
        .unwrap();
}
