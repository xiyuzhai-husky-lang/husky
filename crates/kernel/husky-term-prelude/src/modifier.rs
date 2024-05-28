use husky_coword::Label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VariableModifier {
    Pure,
    Owned,
    Mut,
    Ref,
    RefMut,
    Compterm,
    Ambersand(Option<Label>),
    AmbersandMut(Option<Label>),
    Le,
    Tilde,
    At,
}

impl VariableModifier {
    pub fn new<T>(t: Option<T>) -> Self
    where
        T: Into<Self>,
    {
        match t {
            Some(t) => t.into(),
            None => VariableModifier::Pure,
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
