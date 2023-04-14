use super::*;

pub trait FluffyTermEngine<'a>: Sized {
    fn db(&self) -> &'a dyn FluffyTermDb;
    fn fluffy_term_region(&self) -> &FluffyTermRegion;
    fn fluffy_term_region_mut(&mut self) -> &mut FluffyTermRegion;
    fn fluffy_terms(&self) -> &FluffyTerms {
        self.fluffy_term_region().terms()
    }
    fn term_menu(&self) -> &'a TermMenu;
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
        HollowTerm::new_hole(self, src, hole_kind).into()
    }
}
