pub mod real1d;
pub mod real2d;
pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[
        ("read1d", real1d::SCOPE_DATA),
        ("read2d", real2d::SCOPE_DATA),
    ],
};

use crate::*;
