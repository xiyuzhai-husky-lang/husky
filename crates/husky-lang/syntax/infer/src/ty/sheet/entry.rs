use super::*;
use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TySheetEntry {
    ty: SyntaxResult<ScopePtr>,
    deduction: Deduction,
}
