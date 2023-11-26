use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirRitchieRegularParameter {
    contract: Contract,
    ty: HirType,
}
