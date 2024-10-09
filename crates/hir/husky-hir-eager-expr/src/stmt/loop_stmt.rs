use super::*;
use either::*;
use husky_expr::stmt::{LoopBoundaryKind, LoopStep};
use husky_fly_term::FlyBaseTypeData;
use husky_place::place::{idx::PlaceIdx, EthPlace};
use husky_sem_expr::stmt::loop_stmt::{
    SemForBetweenParticulars, SemaForBetweenLoopBoundary, SemaForBetweenRange,
    SemaForextParticulars,
};
use path::major_item::ty::{PreludeIntTypePath, PreludeNumTypePath, PreludeTypePath};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
pub struct HirEagerForBetweenParticulars {
    pub for_loop_variable_ident: Ident,
    pub for_loop_variable_ty_path: PreludeIntTypePath,
    pub range: HirEagerForBetweenRange,
}

impl ToHirEager for SemForBetweenParticulars {
    type Output = HirEagerForBetweenParticulars;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        let for_loop_variable_ty = self
            .for_between_loop_var_expr_idx()
            .ty(builder.sem_expr_arena_ref2());
        let FlyBaseTypeData::TypeOntology {
            refined_ty_path:
                Left(PreludeTypePath::Num(PreludeNumTypePath::Int(for_loop_variable_ty_path))),
            ..
        } = for_loop_variable_ty.base_ty_data_inner(builder.db(), builder.terms())
        else {
            todo!()
        };
        HirEagerForBetweenParticulars {
            for_loop_variable_ident: self.for_between_loop_var_ident(),
            for_loop_variable_ty_path,
            range: self.range().to_hir_eager(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
pub struct HirEagerForBetweenRange {
    pub initial_boundary: HirEagerForBetweenLoopBoundary,
    pub final_boundary: HirEagerForBetweenLoopBoundary,
    pub step: LoopStep,
}

impl ToHirEager for SemaForBetweenRange {
    type Output = HirEagerForBetweenRange;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerForBetweenRange {
            initial_boundary: self.initial_boundary.to_hir_eager(builder),
            final_boundary: self.final_boundary.to_hir_eager(builder),
            step: self.step,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerForBetweenLoopBoundary {
    pub bound_expr: Option<HirEagerExprIdx>,
    pub kind: LoopBoundaryKind,
}

impl ToHirEager for SemaForBetweenLoopBoundary {
    type Output = HirEagerForBetweenLoopBoundary;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerForBetweenLoopBoundary {
            bound_expr: self.bound_expr.to_hir_eager(builder),
            kind: self.kind,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerForExtParticulars {
    pub forext_loop_var_ident: Ident,
    pub bound_expr_hir_eager_expr_idx: HirEagerExprIdx,
    pub boundary_kind: LoopBoundaryKind,
}

impl ToHirEager for SemaForextParticulars {
    type Output = HirEagerForExtParticulars;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerForExtParticulars {
            forext_loop_var_ident: self.forext_loop_var_ident,
            bound_expr_hir_eager_expr_idx: self.bound_expr.to_hir_eager(builder),
            boundary_kind: self.boundary_kind,
        }
    }
}
