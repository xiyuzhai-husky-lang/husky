use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemPathPresentation {
    ident: String,
    full: String,
}

impl ItemPathPresentation {
    pub fn new(ident: String, full: String) -> Self {
        Self { ident, full }
    }
}

impl ItemPathPresentation {
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
