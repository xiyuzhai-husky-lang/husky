use super::*;

// todo: change to ty_final_destinations: Vec<RawTermSymbolTypeResult<TypeFinalDestination>>,
// RawTerm won't work
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct TermSymbolRegistry {
    ty_final_destinations: Vec<RawTermSymbolTypeResult<RawTerm>>,
}

impl TermSymbolRegistry {
    pub fn new_symbol(
        &mut self,
        db: &dyn RawTermDb,
        ty: RawTermSymbolTypeResult<RawTerm>,
    ) -> RawTermSymbol {
        let idx_usize = self
            .ty_final_destinations
            .iter()
            .filter(|ty1| **ty1 == ty)
            .count();
        let idx = match idx_usize.try_into() {
            Ok(idx) => idx,
            Err(_) => todo!(),
        };
        self.ty_final_destinations.push(ty);
        RawTermSymbol::new(db, ty, idx)
    }
}
