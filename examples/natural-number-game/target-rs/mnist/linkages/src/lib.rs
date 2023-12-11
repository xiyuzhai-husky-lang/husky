use husky_core::*;
use ad_hoc_task_dependency::*;
use mnist::*;

#[rustfmt::skip]
linkages![
    BinaryImage28,
    BinaryGrid28,
    <BinaryImage28 as Visualize>::visualize,
    BinaryImage28::new_zeros,
    <BinaryGrid28 as Visualize>::visualize,
    BinaryGrid28::new_zeros,
];