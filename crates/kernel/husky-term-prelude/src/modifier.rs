use husky_coword::Label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolModifier {
    Pure,
    Owned,
    Mut,
    Ref,
    RefMut,
    Const,
    Ambersand(Option<Label>),
    AmbersandMut(Option<Label>),
    Le,
    Tilde,
    At,
}

impl SymbolModifier {
    pub fn new<T>(t: Option<T>) -> Self
    where
        T: Into<Self>,
    {
        match t {
            Some(t) => t.into(),
            None => SymbolModifier::Pure,
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
