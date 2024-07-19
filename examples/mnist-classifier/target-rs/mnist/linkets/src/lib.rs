#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};
use mnist::*;

#[rustfmt::skip]
linket_impls![
    enum_index_presenter_linket_impl!(mnist::MnistLabel),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Zero),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Zero),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::One),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::One),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Two),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Two),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Three),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Three),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Four),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Four),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Five),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Five),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Six),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Six),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Seven),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Seven),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Eight),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Eight),
    enum_variant_constructor_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Nine),
    enum_variant_discriminator_linket_impl!(mnist::MnistLabel, mnist::MnistLabel::Nine),
    static_var_linket_impl!(mnist::INPUT),
    fn_linket_impl!(<mnist::BinaryImage28>::new_zeros),
    fn_linket_impl!(<mnist::BinaryGrid28>::new_zeros),
    fn_linket_impl!(<mnist::task::MnistTask>::new),
    static_var_linket_impl!(mnist::TASK),
];