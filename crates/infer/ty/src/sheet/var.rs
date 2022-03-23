use scope::InputPlaceholder;
use text::TextRange;

use super::*;
use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TySheetVarEntry {
    pub(super) opt_ty: Option<ScopePtr>,
    pub(super) errors: Vec<InferError>,
}

impl TySheetVarEntry {
    pub(super) fn from_input(input_placeholder: &InputPlaceholder) -> Self {
        TySheetVarEntry {
            opt_ty: Some(input_placeholder.ranged_ty.scope),
            errors: vec![],
        }
    }
}
