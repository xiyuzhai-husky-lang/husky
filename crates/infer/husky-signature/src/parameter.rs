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
        // Ad hoc
        Self {
            pattern: ImplicitParameterSignaturePattern {},
            traits: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterSignatures {
    parameters: Vec<ImplicitParameterSignature>,
}

impl ImplicitParameterSignatures {
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

impl std::ops::Deref for ImplicitParameterSignatures {
    type Target = Vec<ImplicitParameterSignature>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterSignature {
    pattern: ParameterSignaturePattern,
    ty: SignatureOutcome<Term>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterSignaturePattern {}

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterSignatures {
    parameters: Vec<ParameterSignature>,
}

impl ParameterSignatures {
    pub(crate) fn from_decl(
        parameters: &[ParameterDecl],
        engine: &mut SignatureTermEngine,
    ) -> Self {
        Self {
            parameters: parameters
                .iter()
                .map(|parameter| ParameterSignature {
                    pattern: ParameterSignaturePattern {},
                    ty: match engine.query_new(parameter.ty()) {
                        Some(ty) => Success(ty),
                        None => Abort(SignatureAbortion::TermError),
                    },
                })
                .collect(),
        }
    }
}
