use text::TextRange;

use super::*;
use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TySheetVarEntry {
    pub(super) opt_ty: Option<EntityRoutePtr>,
    pub(super) errors: Vec<InferError>,
}

impl TySheetVarEntry {
    pub(super) fn from_parameter(parameter: &Parameter) -> Self {
        TySheetVarEntry {
            opt_ty: Some(parameter.ranged_ty.route),
            errors: vec![],
        }
    }
}
