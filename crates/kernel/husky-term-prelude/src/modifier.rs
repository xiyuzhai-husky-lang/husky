#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolModifier {
    Pure,
    Mut,
    RefMut,
    Const,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldModifier {
    Pure,
    Mut,
    Const,
    Leashed,
}
