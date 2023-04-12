#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolModifier {
    Pure,
    Mut,
    RefMut,
    Const,
}
