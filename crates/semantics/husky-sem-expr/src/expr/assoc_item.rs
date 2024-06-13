use super::*;
use either::*;
use husky_regional_token::IdentRegionalToken;
use maybe_result::*;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_major_item_path_assoc_item_ty(
        &mut self,
        expr_idx: SynExprIdx,
        parent_path: MajorItemPath,
        ident_token: IdentRegionalToken,
    ) -> (
        SemExprDataResult<OntologyDispatch>,
        SemExprTypeResult<FlyTerm>,
    ) {
        let parent_term: FlyTerm = match parent_path {
            MajorItemPath::Type(path) => {
                // ad hoc
                // should consider type template arguments
                ItemPathTerm::TypeOntology(path).into()
            }
            MajorItemPath::Trait(_) => todo!(),
            MajorItemPath::Form(_) => todo!(),
        };
        self.calc_assoc_item_ty_aux(
            parent_term,
            expr_idx,
            ident_token.ident(),
            ident_token.regional_token_idx(),
        )
    }

    pub(super) fn calc_assoc_item_ty(
        &mut self,
        expr_idx: SynExprIdx,
        parent_expr: SemExprIdx,
        colon_colon_regional_token_idx: RegionalTokenIdx,
        ident: Ident,
        ident_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemExprDataResult<OntologyDispatch>,
        SemExprTypeResult<FlyTerm>,
    ) {
        let Some(parent_term) = self.infer_expr_term(parent_expr) else {
            use husky_print_utils::p;
            p!(
                self.syn_expr_region_data().path().debug(self.db()),
                colon_colon_regional_token_idx,
                ident.debug(self.db()),
                parent_expr.data_result(self.sem_expr_arena()),
                self.sem_expr_term_result(parent_expr)
            );
            todo!()
        };
        self.calc_assoc_item_ty_aux(parent_term, expr_idx, ident, ident_regional_token_idx)
    }

    fn calc_assoc_item_ty_aux(
        &mut self,
        parent_term: FlyTerm,
        expr_idx: ArenaIdx<SynExprData>,
        ident: Ident,
        ident_regional_token_idx: RegionalTokenIdx,
    ) -> (
        Result<OntologyDispatch, SemExprDataError>,
        Result<FlyTerm, SemExprTypeError>,
    ) {
        let db = self.db();
        match parent_term.ontology_dispatch(self, expr_idx, ident, /*ad hoc */ &[]) {
            JustOk(ontology_dispatch) => {
                let ty_result = ontology_dispatch.ty_result(self).map_err(Into::into);
                (Ok(ontology_dispatch), ty_result)
            }
            JustErr(_) => todo!(),
            Nothing => todo!(),
        }
    }
}
