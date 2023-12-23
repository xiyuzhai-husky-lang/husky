use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use mnist::*;

#[rustfmt::skip]
linkage_impls![
    fn_linkage_impl!(mnist::BinaryImage28),
    fn_linkage_impl!(mnist::BinaryGrid28),
    fn_linkage_impl!(mnist::input),
    fn_linkage_impl!(<mnist::BinaryImage28>::new_zeros),
    fn_linkage_impl!(<mnist::BinaryGrid28>::new_zeros),
];