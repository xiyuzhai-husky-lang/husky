use husky_expr::ImplicitParameterDeclPatternVariant;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitParameterSignature {
    term_symbol: TermSymbol,
    ty: SignatureTermOutcome<Term>,
    traits: Vec<Term>,
}

impl ImplicitParameterSignature {
    fn from_decl(
        parameter: &ImplicitParameterDecl,
        engine: &mut SignatureTermEngine,
    ) -> ImplicitParameterSignature {
        let symbol = parameter.pattern().symbol();
        let variant = parameter.pattern().variant();
        match variant {
            ImplicitParameterDeclPatternVariant::Type0 { .. } => {
                ImplicitParameterSignature {
                    term_symbol: engine.current_symbol_term_symbol(symbol),
                    ty: Success(engine.term_menu().ty0()),
                    // ad hoc
                    traits: vec![],
                }
            }
            ImplicitParameterDeclPatternVariant::Constant => todo!(),
            ImplicitParameterDeclPatternVariant::Lifetime => todo!(),
            ImplicitParameterDeclPatternVariant::Binding => todo!(),
        }
    }

    pub fn term_symbol(&self) -> TermSymbol {
        self.term_symbol
    }

    pub fn ty(&self) -> SignatureTermOutcomeBorrowed<Term> {
        self.ty.ok_copy_err_as_ref()
    }

    pub fn traits(&self) -> &[Term] {
        self.traits.as_ref()
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
    ty: SignatureTermOutcome<Term>,
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
                    ty: engine.query_new(parameter.ty()),
                })
                .collect(),
        }
    }
}
