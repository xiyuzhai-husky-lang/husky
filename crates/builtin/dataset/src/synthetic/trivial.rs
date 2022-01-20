pub mod real1d;
pub mod real2d;
pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[
        ("real1d", real1d::SCOPE_DATA),
        ("real2d", real2d::SCOPE_DATA),
    ],
    signature: ScopeSignature::Module,
};

use crate::*;
