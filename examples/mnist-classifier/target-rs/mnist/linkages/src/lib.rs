#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use mnist::*;

#[rustfmt::skip]
linkage_impls![
    enum_u8_presenter_linkage_impl!(mnist::MnistLabel),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::Zero),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__Zero_discriminator),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::One),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__One_discriminator),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::Two),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__Two_discriminator),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::Three),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__Three_discriminator),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::Four),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__Four_discriminator),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::Five),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__Five_discriminator),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::Six),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__Six_discriminator),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::Seven),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__Seven_discriminator),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::Eight),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__Eight_discriminator),
    enum_variant_unit_constructor_linkage_impl!(mnist::MnistLabel::Nine),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::__Nine_discriminator),
    fn_linkage_impl!(mnist::input),
    fn_linkage_impl!(<mnist::BinaryImage28>::new_zeros),
    fn_linkage_impl!(<mnist::BinaryGrid28>::new_zeros),
];