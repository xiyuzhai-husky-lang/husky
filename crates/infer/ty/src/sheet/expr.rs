use entity_route::InputPlaceholder;
use text::TextRange;

use super::*;
use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TySheetExprEntry {
    pub(crate) opt_ty: Option<EntityRoutePtr>,
    pub(crate) errors: Vec<InferError>,
    pub(crate) range: TextRange,
}

impl From<&RawExpr> for TySheetExprEntry {
    fn from(expr: &RawExpr) -> Self {
        Self {
            opt_ty: None,
            errors: vec![],
            range: expr.range,
        }
    }
}
