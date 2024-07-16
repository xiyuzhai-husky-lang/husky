#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};
use mnist::*;

#[rustfmt::skip]
linkage_impls![
    enum_index_presenter_linkage_impl!(mnist::MnistLabel),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Zero),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Zero),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::One),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::One),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Two),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Two),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Three),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Three),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Four),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Four),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Five),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Five),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Six),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Six),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Seven),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Seven),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Eight),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Eight),
    enum_variant_constructor_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Nine),
    enum_variant_discriminator_linkage_impl!(mnist::MnistLabel, mnist::MnistLabel::Nine),
    static_var_linkage_impl!(mnist::INPUT),
    fn_linkage_impl!(<mnist::BinaryImage28>::new_zeros),
    fn_linkage_impl!(<mnist::BinaryGrid28>::new_zeros),
    fn_linkage_impl!(<mnist::task::MnistTask>::new),
    static_var_linkage_impl!(mnist::TASK),
];