use crate::utils::RawTermFamily;

use super::*;
use husky_entity_path::TypePath;

// todo: change to ty_final_destinations: Vec<RawTermSymbolTypeResult<TypeFinalDestination>>,
// RawTerm won't work
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct RawTermSymbolRegistry {
    // ad hoc
    ty_families: Vec<RawTermFamily>,
}

impl RawTermSymbolRegistry {
    pub fn new_symbol(
        &mut self,
        db: &dyn RawTermDb,
        ty: RawTermSymbolTypeResult<RawTerm>,
    ) -> RawTermSymbol {
        let ty_path = ty.map(|ty| ty.family(db)).unwrap_or(RawTermFamily::Other);
        let idx_usize = self
            .ty_families
            .iter()
            .filter(|ty_path1| **ty_path1 == ty_path)
            .count();
        let idx = match idx_usize.try_into() {
            Ok(idx) => idx,
            Err(_) => todo!(),
        };
        self.ty_families.push(ty_path);
        RawTermSymbol::new(db, ty, idx)
    }
}
