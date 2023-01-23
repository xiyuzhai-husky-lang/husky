use husky_expr::{ImplicitParameterDeclPatternVariant, RegularParameterDeclPattern};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterSignature {
    term_symbol: TermSymbol,
    ty: Term,
    traits: Vec<Term>,
}

impl ImplicitParameterSignature {
    fn from_decl(
        parameter: &ImplicitParameterDecl,
        region: &SignatureTermRegion,
        term_menu: &TermMenu,
    ) -> ImplicitParameterSignature {
        let symbol = parameter.pattern().symbol();
        let variant = parameter.pattern().variant();
        match variant {
            ImplicitParameterDeclPatternVariant::Type0 { .. } => {
                ImplicitParameterSignature {
                    term_symbol: region.current_symbol_term(symbol),
                    ty: term_menu.ty0(),
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

    pub fn ty(&self) -> Term {
        self.ty
    }

    pub fn traits(&self) -> &[Term] {
        self.traits.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterSignatures {
    parameters: Vec<ImplicitParameterSignature>,
}

impl ImplicitParameterSignatures {
    pub(crate) fn from_decl(
        parameters: &[ImplicitParameterDecl],
        signature_term_region: &SignatureTermRegion,
        term_menu: &TermMenu,
    ) -> Self {
        Self {
            parameters: parameters
                .iter()
                .map(|parameter| {
                    ImplicitParameterSignature::from_decl(
                        parameter,
                        signature_term_region,
                        term_menu,
                    )
                })
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignature {
    pattern: ParameterSignaturePattern,
    ty: Term,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignaturePattern {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignatures {
    parameters: Vec<ParameterSignature>,
}

impl ParameterSignatures {
    pub(crate) fn from_decl(
        parameters: &[RegularParameterDeclPattern],
        sheet: &SignatureTermRegion,
    ) -> SignatureResult<Self> {
        Ok(Self {
            parameters: parameters
                .iter()
                .map(|parameter| {
                    let ty = parameter.ty();
                    let ty = match sheet.expr_term(ty) {
                        Ok(ty) => ty,
                        Err(_) => todo!(),
                    };
                    Ok(ParameterSignature {
                        pattern: ParameterSignaturePattern {},
                        ty,
                    })
                })
                .collect::<Result<Vec<ParameterSignature>, SignatureError>>()?,
        })
    }
}
