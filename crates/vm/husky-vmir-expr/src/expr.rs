use crate::ToVmir;
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprIdx};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub enum VmirExprData {
    Variable,
}

pub type VmirExprArena = Arena<VmirExprData>;
pub type VmirExprIdx = ArenaIdx<VmirExprData>;
pub type VmirExprIdxRange = ArenaIdxRange<VmirExprData>;

impl ToVmir for HirEagerExprIdx {
    type Output = VmirExprIdx;

    fn to_vmir(self, builder: &mut crate::builder::VmirExprBuilder) -> Self::Output {
        match *builder.hir_eager_expr_arena()[self].data() {
            HirEagerExprData::Literal(_) => todo!(),
            HirEagerExprData::PrincipalEntityPath(_) => todo!(),
            HirEagerExprData::AssocFn { assoc_item_path } => todo!(),
            HirEagerExprData::ConstSvar { ident } => todo!(),
            HirEagerExprData::Variable(_) => todo!(),
            HirEagerExprData::Binary { lopd, opr, ropd } => todo!(),
            HirEagerExprData::Be { src, ref target } => todo!(),
            HirEagerExprData::Prefix {
                opr,
                opd_hir_expr_idx,
            } => todo!(),
            HirEagerExprData::Suffix {
                opd_hir_expr_idx,
                opr,
            } => todo!(),
            HirEagerExprData::Unveil {
                unveil_assoc_fn_path,
                ref instantiation,
                return_ty,
                opd_hir_expr_idx,
            } => todo!(),
            HirEagerExprData::Unwrap { opd_hir_expr_idx } => todo!(),
            HirEagerExprData::As { opd, ty } => todo!(),
            HirEagerExprData::TypeConstructorFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::TypeVariantConstructorCall {
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::FunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::AssocFunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::PropsStructField {
                owner_hir_expr_idx,
                ident,
                field_ty,
            } => todo!(),
            HirEagerExprData::MemoizedField {
                owner_hir_expr_idx,
                ident,
                path,
            } => todo!(),
            HirEagerExprData::MethodFnCall {
                self_argument,
                self_contract,
                ident,
                path,
                ref instantiation,
                ref item_groups,
            } => todo!(),
            HirEagerExprData::NewTuple { ref items } => todo!(),
            HirEagerExprData::Index {
                owner_hir_expr_idx,
                ref items,
            } => todo!(),
            HirEagerExprData::NewList {
                ref items,
                element_ty,
            } => todo!(),
            HirEagerExprData::Block { stmts } => todo!(),
            HirEagerExprData::Closure {
                ref parameters,
                return_ty,
                body,
            } => todo!(),
            HirEagerExprData::EmptyHtmlTag {
                function_ident,
                ref arguments,
            } => todo!(),
            HirEagerExprData::Todo => todo!(),
            HirEagerExprData::Unreachable => todo!(),
        }
    }
}
