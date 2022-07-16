use husky_infer_qualified_ty::{EagerValueQualifiedTy, EagerVariableQualifiedTy, QualifiedTySheet};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerVariable {
    pub ident: CustomIdentifier,
    pub qualified_ty: EagerVariableQualifiedTy,
}

impl EagerVariable {
    pub(crate) fn from_parameter(
        qualified_ty_sheet: &QualifiedTySheet,
        parameter: &Parameter,
    ) -> SemanticResult<Self> {
        Ok(EagerVariable {
            ident: parameter.ranged_ident.ident,
            qualified_ty: qualified_ty_sheet.eager_variable_qualified_ty(
                parameter.ranged_ident.ident.into(),
                parameter.ranged_ident.range,
            )?,
        })
    }
}
