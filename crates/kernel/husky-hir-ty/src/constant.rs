use crate::*;

#[salsa::debug_with_db(db = HirTypeDb)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirConstant {}
