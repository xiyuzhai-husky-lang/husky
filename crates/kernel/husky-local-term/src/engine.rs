use crate::*;
use husky_signature::SymbolSignature;

pub trait LocalTermEngine<'a> {
    fn db(&self) -> &'a dyn TermDb;
    fn local_term_region(&self) -> &LocalTermRegion;
    fn local_term_region_mut(&mut self) -> &mut LocalTermRegion;
    fn unresolved_terms(&self) -> &UnresolvedTerms {
        self.local_term_region().unresolved_terms()
    }
    fn expr_region_data(&self) -> &'a ExprRegionData;

    fn new_ty_ontology_application(
        &mut self,
        src_expr_idx: ExprIdx,
        path: TypePath,
        arguments: SmallVec<[LocalTerm; 2]>,
    ) -> LocalTerm {
        todo!()
    }

    #[inline(always)]
    fn new_qualified_ty(
        &mut self,
        symbol_idx: impl IntoLocalSymbolIdx,
        signature: SymbolSignature,
    ) -> TermResult<PlaceTypeIdx> {
        let local_symbol_idx = symbol_idx.into_local_symbol_idx(self.expr_region_data());
        let place = match signature.modifier() {
            SymbolModifier::Pure => todo!(),
            SymbolModifier::Mut => todo!(),
            SymbolModifier::Const => Place::Const, // todo: handle variance
        };
        let ty = Term::ty_from_raw(self.db(), signature.ty()?)?;
        Ok(PlaceType::new(self.local_term_region_mut(), place, ty))
    }
}
