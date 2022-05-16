use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ContextualIdentifier {
    Package,
    Input,
    ThisData,
    ThisType,
}

impl ContextualIdentifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContextualIdentifier::Package => "package",
            ContextualIdentifier::Input => "input",
            ContextualIdentifier::ThisData => "this",
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
