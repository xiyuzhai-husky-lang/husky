use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// #[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct HirRitchieKeyedParameter {
    key: Ident,
    contract: Contract,
    ty: HirType,
    default: bool,
}
