pub mod math;
pub mod text;

use self::{math::TexMathTokenData, text::TexTextTokenData};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexTokenData {
    Math(TexMathTokenData),
    Text(TexTextTokenData),
}
