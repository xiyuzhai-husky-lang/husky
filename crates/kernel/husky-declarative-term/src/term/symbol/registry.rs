use crate::utils::DeclarativeTermFamily;

use super::*;
use husky_entity_path::TypePath;

// todo: change to ty_final_destinations: Vec<DeclarativeTermSymbolTypeResult<TypeFinalDestination>>,
// DeclarativeTerm won't work
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct DeclarativeTermSymbolRegistry {
    // ad hoc
    ty_families: Vec<DeclarativeTermFamily>,
}

impl DeclarativeTermSymbolRegistry {
    pub fn new_symbol(
        &mut self,
        db: &dyn DeclarativeTermDb,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    ) -> DeclarativeTermSymbol {
        let ty_path = ty
            .map(|ty| ty.family(db))
            .unwrap_or(DeclarativeTermFamily::Other);
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
        DeclarativeTermSymbol::new(db, ty, idx)
    }
}
