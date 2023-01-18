use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterSignaturePattern {}

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterSignature {
    pattern: ImplicitParameterSignaturePattern,
    traits: Vec<Term>,
}

impl ImplicitParameterSignature {
    pub fn pattern(&self) -> &ImplicitParameterSignaturePattern {
        &self.pattern
    }

    fn from_decl(
        parameter: &ImplicitParameterDecl,
        engine: &mut SignatureTermEngine,
    ) -> ImplicitParameterSignature {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterSignatureList {
    parameters: Vec<ImplicitParameterSignature>,
}

impl ImplicitParameterSignatureList {
    pub(crate) fn from_decl(
        parameters: &[ImplicitParameterDecl],
        engine: &mut SignatureTermEngine,
    ) -> Self {
        Self {
            parameters: parameters
                .iter()
                .map(|parameter| ImplicitParameterSignature::from_decl(parameter, engine))
                .collect(),
        }
    }

    pub fn decls(&self) -> &[ImplicitParameterSignature] {
        self.parameters.as_ref()
    }
}

impl std::ops::Deref for ImplicitParameterSignatureList {
    type Target = Vec<ImplicitParameterSignature>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterSignature {
    pattern: ParameterSignaturePattern,
    ty: Term,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterSignaturePattern {}

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterSignatureList {
    decls: Vec<ParameterSignature>,
}
