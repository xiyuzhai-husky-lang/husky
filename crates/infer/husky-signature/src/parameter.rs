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
        sheet: &SignatureTermRegion,
    ) -> ImplicitParameterSignature {
        todo!()
        // let symbol = parameter.pattern().symbol();
        // let variant = parameter.pattern().variant();
        // match variant {
        //     ImplicitParameterDeclPatternVariant::Type0 { .. } => {
        //         ImplicitParameterSignature {
        //             term_symbol: sheet.current_symbol_term_symbol(symbol),
        //             ty: Success(sheet.term_menu().ty0()),
        //             // ad hoc
        //             traits: vec![],
        //         }
        //     }
        //     ImplicitParameterDeclPatternVariant::Constant => todo!(),
        //     ImplicitParameterDeclPatternVariant::Lifetime => todo!(),
        //     ImplicitParameterDeclPatternVariant::Binding => todo!(),
        // }
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
        sheet: &SignatureTermRegion,
    ) -> Self {
        Self {
            parameters: parameters
                .iter()
                .map(|parameter| ImplicitParameterSignature::from_decl(parameter, sheet))
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
    ) -> Self {
        Self {
            parameters: parameters
                .iter()
                .map(|parameter| ParameterSignature {
                    pattern: ParameterSignaturePattern {},
                    ty: todo!(),
                })
                .collect(),
        }
    }
}
