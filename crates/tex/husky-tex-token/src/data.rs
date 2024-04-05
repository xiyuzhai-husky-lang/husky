pub mod math;
pub mod text;

use self::{math::TexMathTokenData, text::TexTextTokenData};

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexTokenData {
    Math(TexMathTokenData),
    Text(TexTextTokenData),
}
