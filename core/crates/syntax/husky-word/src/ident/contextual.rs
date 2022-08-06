use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ContextualIdentifier {
    Crate,
    InputValue,
    OutputType,
    ThisValue,
    ThisType,
}
impl From<ContextualIdentifier> for Identifier {
    fn from(ident: ContextualIdentifier) -> Self {
        Self::Contextual(ident)
    }
}

impl ContextualIdentifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContextualIdentifier::Crate => "crate",
            ContextualIdentifier::InputValue => "input",
            ContextualIdentifier::OutputType => "Output",
            ContextualIdentifier::ThisValue => "this",
            ContextualIdentifier::ThisType => "This",
        }
    }
}

impl Deref for ContextualIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for ContextualIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}
