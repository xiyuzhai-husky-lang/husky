use super::*;
use husky_coword::Ident;
use husky_expr::stmt::{LoopBoundaryKind, LoopStep};
use husky_opr::BinaryComparisonOpr;
use husky_regional_token::RegionalTokenIdx;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb, jar = SemaExprJar)]
pub struct SemaForBetweenParticulars {
    for_between_loop_var_regional_token_idx: RegionalTokenIdx,
    for_between_loop_var_ident: Ident,
    for_between_loop_var_expr_idx: SemaExprIdx,
    range: SemaForBetweenRange,
}

impl SemaForBetweenParticulars {
    pub fn for_between_loop_var_regional_token_idx(&self) -> RegionalTokenIdx {
        self.for_between_loop_var_regional_token_idx
    }

    pub fn for_between_loop_var_ident(&self) -> Ident {
        self.for_between_loop_var_ident
    }

    pub fn for_between_loop_var_expr_idx(&self) -> SemaExprIdx {
        self.for_between_loop_var_expr_idx
    }

    pub fn range(&self) -> &SemaForBetweenRange {
        &self.range
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_for_between_particulars(
        &mut self,
        particulars: &'a SynForBetweenParticulars,
        for_loop_var_symbol_idx: CurrentSynSymbolIdx,
    ) -> SynExprResultRef<'a, SemaForBetweenParticulars> {
        let Ok(ref range) = particulars.range else {
            todo!()
        };
        let mut expected_frame_var_ty: Option<FluffyTerm> = None;
        let initial_bound_sema_expr_idx = match range.initial_boundary.bound_expr {
            Some(bound_expr) => {
                let (bound_sema_expr_idx, num_ty_outcome) =
                    self.build_sema_expr_with_outcome(bound_expr, ExpectIntType);
                match num_ty_outcome {
                    Some(num_ty_outcome) => {
                        expected_frame_var_ty = Some(num_ty_outcome.placeless_num_ty())
                    }
                    None => (),
                };
                Some(bound_sema_expr_idx)
            }
            None => None,
        };
        let final_bound_sema_expr_idx = match range.final_boundary.bound_expr {
            Some(bound_expr) => match expected_frame_var_ty {
                Some(expected_frame_var_ty) => Some(self.build_sema_expr(
                    bound_expr,
                    ExpectCoersion::new_pure(self, expected_frame_var_ty),
                )),
                None => {
                    let (final_bound_sema_expr_idx, ty) =
                        self.build_sema_expr_with_its_ty_returned(bound_expr, ExpectAnyOriginal);
                    if let Some(ty) = ty {
                        expected_frame_var_ty = Some(ty)
                    }
                    Some(final_bound_sema_expr_idx)
                }
            },
            None => None,
        };
        let Some(expected_frame_var_ty) = expected_frame_var_ty else {
            todo!()
        };
        let place = FluffyPlace::ImmutableStackOwned {
            location: for_loop_var_symbol_idx
                .into_local_symbol_idx(self.syn_expr_region_data())
                .into(),
        };
        let frame_var_symbol_ty =
            SymbolType::new(self, for_loop_var_symbol_idx, expected_frame_var_ty);
        self.add_symbol_ty(for_loop_var_symbol_idx, frame_var_symbol_ty);
        let for_between_loop_var_expr_idx = self.build_sema_expr(
            particulars.for_between_loop_var_expr_idx,
            ExpectCoersion::new_pure(self, frame_var_symbol_ty.term()),
        );
        let range = SemaForBetweenRange {
            initial_boundary: SemaForBetweenLoopBoundary {
                bound_expr: initial_bound_sema_expr_idx,
                kind: range.initial_boundary.kind,
            },
            final_boundary: SemaForBetweenLoopBoundary {
                bound_expr: final_bound_sema_expr_idx,
                kind: range.final_boundary.kind,
            },
            step: range.step,
        };
        Ok(SemaForBetweenParticulars {
            for_between_loop_var_regional_token_idx: particulars
                .for_between_loop_var_regional_token_idx,
            for_between_loop_var_ident: particulars.for_between_loop_var_ident,
            for_between_loop_var_expr_idx,
            range,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb, jar = SemaExprJar)]
pub struct SemaForBetweenRange {
    pub initial_boundary: SemaForBetweenLoopBoundary,
    pub final_boundary: SemaForBetweenLoopBoundary,
    pub step: LoopStep,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SemaForBetweenLoopBoundary {
    pub bound_expr: Option<SemaExprIdx>,
    pub kind: LoopBoundaryKind,
}

impl Default for SemaForBetweenLoopBoundary {
    fn default() -> Self {
        Self {
            bound_expr: None,
            kind: LoopBoundaryKind::LowerClosed,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaForextParticulars {
    pub forext_loop_var_regional_token_idx: RegionalTokenIdx,
    pub forext_loop_var_ident: Ident,
    pub forext_loop_var_sema_expr_idx: SemaExprIdx,
    pub bound_expr_sema_expr_idx: SemaExprIdx,
    pub boundary_kind: LoopBoundaryKind,
}

impl SemaForextParticulars {
    pub(crate) fn new(
        forext_loop_var_regional_token_idx: RegionalTokenIdx,
        forext_loop_var_ident: Ident,
        forext_loop_var_expr_idx: SemaExprIdx,
        opr: BinaryComparisonOpr,
        bound_expr: SemaExprIdx,
    ) -> Self {
        todo!()
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_forext_particulars(
        &mut self,
        particulars: &'a SynForextParticulars,
        forext_loop_var_sema_expr_idx: SemaExprIdx,
        forext_loop_var_ty: FluffyTerm,
    ) -> SemaForextParticulars {
        let bound_expr_sema_expr_idx = self.build_sema_expr(
            particulars.bound_expr,
            ExpectCoersion::new_pure(self, forext_loop_var_ty),
        );
        SemaForextParticulars {
            forext_loop_var_regional_token_idx: particulars.forext_loop_var_regional_token_idx,
            forext_loop_var_ident: particulars.forext_loop_var_ident,
            forext_loop_var_sema_expr_idx,
            bound_expr_sema_expr_idx,
            boundary_kind: particulars.boundary_kind,
        }
    }
}
