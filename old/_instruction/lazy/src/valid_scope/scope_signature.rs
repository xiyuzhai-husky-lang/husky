use syntax_types::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeSignature {
    Variadic(usize),
    Fixed(Vec<SpaceParamKind>),
}

impl From<Vec<SpaceParamKind>> for ScopeSignature {
    fn from(v: Vec<SpaceParamKind>) -> Self {
        ScopeSignature::Fixed(v)
    }
}

impl Default for ScopeSignature {
    fn default() -> Self {
        Self::Fixed(Vec::new())
    }
}
