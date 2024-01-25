use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use ot::prelude::*;
use rust_optimal_transport as ot; // max()

#[test]
fn try_ot() {
    // Generate data by sampling a 2D gaussian distribution
    let n = 100;

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
