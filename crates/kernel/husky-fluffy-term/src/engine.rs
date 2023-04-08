use super::*;

pub trait FluffyTermEngine<'a> {
    fn db(&self) -> &'a dyn FluffyTermDb;
    fn fluffy_term_region(&self) -> &FluffyTermRegion;
    fn fluffy_term_region_mut(&mut self) -> &mut FluffyTermRegion;
    fn fluffy_terms(&self) -> &FluffyTerms {
        self.fluffy_term_region().terms()
    }
    fn expr_region_data(&self) -> &'a ExprRegionData;

    fn new_ty_ontology_application(
        &mut self,
        src: HollowTermSource,
        path: TypePath,
        arguments: SmallVec<[FluffyTerm; 2]>,
    ) -> FluffyTerm {
        todo!()
    }

    #[inline(always)]
    fn new_place_ty(
        &mut self,
        symbol_idx: impl IntoLocalSymbolIdx,
        signature: SymbolSignature,
    ) -> TermResult<PlaceType> {
        todo!()
        // let local_symbol_idx = symbol_idx.into_local_symbol_idx(self.expr_region_data());
        // let place = match signature.modifier() {
        //     SymbolModifier::Pure => todo!(),
        //     SymbolModifier::Mut => todo!(),
        //     SymbolModifier::Const => Place::Const, // todo: handle variance
        // };
        // let ty = Term::ty_from_raw(self.db(), signature.ty()?)?;
        // Ok(PlaceType::new(self.fluffy_term_region_mut(), place, ty))
    }
}
