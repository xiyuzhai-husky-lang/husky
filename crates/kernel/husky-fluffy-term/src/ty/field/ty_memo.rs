use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyTypeMemoFieldDisambiguation {
    indirections: Vec<FluffyFieldIndirection>,
}

impl FluffyTerm {
    pub(super) fn ty_memo_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
    ) -> FluffyTypeResult<
        Option<(
            FluffyTypeMemoFieldDisambiguation,
            FluffyTypeResult<FluffyTerm>,
        )>,
    > {
        // todo!()
        Ok(None)
    }
}
