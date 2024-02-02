use husky_coword::{Ident, Label};

#[enum_class::from_variants]
#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolName {
    Ident(Ident),
    Label(Label),
    SelfType,
    SelfValue,
}
