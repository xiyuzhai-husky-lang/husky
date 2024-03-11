#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use mnist::*;

#[rustfmt::skip]
linkage_impls![
    enum_u8_presenter_linkage_impl!(mnist::MnistLabel),
    fn_linkage_impl!(mnist::MnistLabel::Zero),
    fn_linkage_impl!(mnist::MnistLabel::__Zero_discriminator),
    fn_linkage_impl!(mnist::MnistLabel::One),
    fn_linkage_impl!(mnist::MnistLabel::__One_discriminator),
    fn_linkage_impl!(mnist::MnistLabel::Two),
    fn_linkage_impl!(mnist::MnistLabel::__Two_discriminator),
    fn_linkage_impl!(mnist::MnistLabel::Three),
    fn_linkage_impl!(mnist::MnistLabel::__Three_discriminator),
    fn_linkage_impl!(mnist::MnistLabel::Four),
    fn_linkage_impl!(mnist::MnistLabel::__Four_discriminator),
    fn_linkage_impl!(mnist::MnistLabel::Five),
    fn_linkage_impl!(mnist::MnistLabel::__Five_discriminator),
    fn_linkage_impl!(mnist::MnistLabel::Six),
    fn_linkage_impl!(mnist::MnistLabel::__Six_discriminator),
    fn_linkage_impl!(mnist::MnistLabel::Seven),
    fn_linkage_impl!(mnist::MnistLabel::__Seven_discriminator),
    fn_linkage_impl!(mnist::MnistLabel::Eight),
    fn_linkage_impl!(mnist::MnistLabel::__Eight_discriminator),
    fn_linkage_impl!(mnist::MnistLabel::Nine),
    fn_linkage_impl!(mnist::MnistLabel::__Nine_discriminator),
    fn_linkage_impl!(mnist::input),
    fn_linkage_impl!(<mnist::BinaryImage28>::new_zeros),
    fn_linkage_impl!(<mnist::BinaryGrid28>::new_zeros),
];