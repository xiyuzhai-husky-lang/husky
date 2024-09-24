use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemDomainStaticMutDeps(OrderedSmallVecSet<ItemPath, 4>);
