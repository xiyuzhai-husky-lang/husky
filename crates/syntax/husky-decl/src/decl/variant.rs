mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VariantDecl {
    Props(PropsVariantDecl),
    Unit(UnitVariantDecl),
    Tuple(TupleVariantDecl),
}
