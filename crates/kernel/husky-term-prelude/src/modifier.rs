#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolModifier {
    None,
    Mut,
    RefMut,
    Const,
    Ambersand(Option<husky_coword::Label>),
    AmbersandMut(Option<husky_coword::Label>),
    Le,
    Tilde,
}

impl SymbolModifier {
    pub fn new<T>(t: Option<T>) -> Self
    where
        T: Into<Self>,
    {
        match t {
            Some(t) => t.into(),
            None => SymbolModifier::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldModifier {
    Pure,
    Mut,
    Const,
    Leashed,
}
