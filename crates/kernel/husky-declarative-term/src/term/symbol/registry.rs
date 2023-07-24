use crate::helpers::DeclarativeTermFamily;

use super::*;
use husky_item_path::TypePath;

// todo: change to ty_final_destinations: Vec<DeclarativeTermSymbolTypeResult<TypeFinalDestination>>,
// DeclarativeTerm won't work
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct DeclarativeTermSymbolRegistry {
    // ad hoc
    families: Vec<DeclarativeTermFamily>,
}

impl DeclarativeTermSymbolRegistry {
    pub fn new_symbol(
        &mut self,
        db: &dyn DeclarativeTermDb,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    ) -> DeclarativeTermSymbol {
        let family = ty
            .map(|ty| ty.family(db))
            .unwrap_or(DeclarativeTermFamily::Other);
        let idx_usize = self
            .families
            .iter()
            .filter(|family1| **family1 == family)
            .count();
        let idx = match idx_usize.try_into() {
            Ok(idx) => idx,
            Err(_) => todo!(),
        };
        self.families.push(family);
        DeclarativeTermSymbol::new(db, ty, idx)
    }
}
