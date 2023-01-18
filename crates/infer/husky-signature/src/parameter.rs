use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterSignaturePattern {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterSignature {
    pattern: ImplicitParameterSignaturePattern,
    traits: Vec<Term>,
}

impl ImplicitParameterSignature {
    pub fn pattern(&self) -> &ImplicitParameterSignaturePattern {
        &self.pattern
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterSignatureList {
    decls: Vec<ImplicitParameterSignature>,
}

impl std::ops::Deref for ImplicitParameterSignatureList {
    type Target = Vec<ImplicitParameterSignature>;

    fn deref(&self) -> &Self::Target {
        &self.decls
    }
}

impl ImplicitParameterSignatureList {
    pub fn decls(&self) -> &[ImplicitParameterSignature] {
        self.decls.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignature {
    pattern: ParameterSignaturePattern,
    ty: Term,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignaturePattern {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignatureList {
    decls: Vec<ParameterSignature>,
}
