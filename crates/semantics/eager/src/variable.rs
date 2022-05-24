use infer_qualifier::{EagerQualifiedTy, QualifiedTySheet};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerVariable {
    pub ident: CustomIdentifier,
    pub qualified_ty: EagerQualifiedTy,
}

impl EagerVariable {
    pub(crate) fn from_input(
        qualified_ty_sheet: &QualifiedTySheet,
        input_placeholder: &InputParameter,
    ) -> SemanticResult<Self> {
        Ok(EagerVariable {
            ident: input_placeholder.ranged_ident.ident,
            qualified_ty: qualified_ty_sheet.eager_variable_qualified_ty(
                input_placeholder.ranged_ident.ident.into(),
                input_placeholder.ranged_ident.range,
            )?,
        })
    }
}
