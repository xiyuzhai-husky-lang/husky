use super::*;

pub trait FluffyTermEngine<'a>: Sized {
    fn db(&self) -> &'a dyn FluffyTermDb;
    fn fluffy_term_region(&self) -> &FluffyTermRegion;
    fn fluffy_term_region_mut(&mut self) -> &mut FluffyTermRegion;
    fn fluffy_terms(&self) -> &FluffyTerms {
        self.fluffy_term_region().terms()
    }
    fn expr_region_data(&self) -> &'a ExprRegionData;

    fn new_ty_ontology_application(
        &mut self,
        src: HoleSource,
        path: TypePath,
        arguments: SmallVec<[FluffyTerm; 2]>,
    ) -> FluffyTerm {
        todo!()
    }

    fn new_hole(&mut self, src: impl Into<HoleSource>, hole_kind: HoleKind) -> FluffyTerm {
        HollowTerm::new_hole(
            self.fluffy_term_region_mut().hollow_terms_mut(),
            src,
            hole_kind,
        )
        .into()
    }

    #[inline(always)]
    fn new_place_ty(
        &mut self,
        symbol_idx: impl IntoLocalSymbolIdx,
        signature: SymbolSignature,
    ) -> TermResult<FluffyTerm> {
        let local_symbol_idx = symbol_idx.into_local_symbol_idx(self.expr_region_data());
        let place = match signature.modifier() {
            SymbolModifier::Pure => Place::StackPure {
                location: local_symbol_idx.into(),
            },
            SymbolModifier::Mut => Place::MutableStackOwned {
                location: local_symbol_idx.into(),
            },
            SymbolModifier::RefMut => Place::RefMut {
                guard: Left(local_symbol_idx.into()),
            },
            SymbolModifier::Const => Place::Const, // todo: handle variance
        };
        let ty = Term::ty_from_raw(self.db(), signature.ty()?)?;
        Ok(FluffyTerm::new_place_ty(self, place, ty.into()))
    }
}
