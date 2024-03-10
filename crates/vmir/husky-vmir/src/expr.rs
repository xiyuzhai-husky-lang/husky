use crate::{
    builder::VmirExprBuilder, destroyer::VmirDestroyerIdxRange, stmt::VmirStmtIdxRange, ToVmir,
};
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprIdx};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirExprData {
    Literal,
    Variable,
    Binary,
    Be,
    Prefix,
    Suffix,
    Unveil,
    Linkage,
    Block {
        stmts: VmirStmtIdxRange,
        destroyers: VmirDestroyerIdxRange,
    },
    Closure,
    Todo,
    Unreachable,
}

pub type VmirExprArena = Arena<VmirExprData>;
pub type VmirExprIdx = ArenaIdx<VmirExprData>;
pub type VmirExprIdxRange = ArenaIdxRange<VmirExprData>;

impl ToVmir for HirEagerExprIdx {
    type Output = VmirExprIdx;

    fn to_vmir(self, builder: &mut VmirExprBuilder) -> Self::Output {
        let expr_data = match *builder.hir_eager_expr_arena()[self].data() {
            HirEagerExprData::Literal(_) => VmirExprData::Literal,
            HirEagerExprData::PrincipalEntityPath(_) => todo!(),
            HirEagerExprData::AssocFn { assoc_item_path } => todo!(),
            HirEagerExprData::ConstSvar { ident } => todo!(),
            HirEagerExprData::Variable(_) => VmirExprData::Variable,
            HirEagerExprData::Binary { lopd, opr, ropd } => VmirExprData::Binary,
            HirEagerExprData::Be { src, ref target } => VmirExprData::Be,
            HirEagerExprData::Prefix { opr, opd } => VmirExprData::Prefix,
            HirEagerExprData::Suffix { opd, opr } => VmirExprData::Suffix,
            HirEagerExprData::Unveil {
                unveil_assoc_fn_path,
                ref instantiation,
                return_ty,
                opd,
            } => VmirExprData::Unveil,
            HirEagerExprData::Unwrap { opd } => todo!(),
            HirEagerExprData::As { opd, ty } => todo!(),
            HirEagerExprData::TypeConstructorFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage,
            HirEagerExprData::TypeVariantConstructorCall {
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage,
            HirEagerExprData::FunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage,
            HirEagerExprData::AssocFunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage,
            HirEagerExprData::PropsStructField {
                owner,
                ident,
                field_ty,
            } => VmirExprData::Linkage,
            HirEagerExprData::MemoizedField {
                owner_hir_expr_idx,
                ident,
                path,
            } => VmirExprData::Linkage,
            HirEagerExprData::MethodFnCall {
                self_argument,
                self_contract,
                ident,
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage,
            HirEagerExprData::NewTuple { .. } => VmirExprData::Linkage,
            HirEagerExprData::Index { owner, ref items } => VmirExprData::Linkage,
            HirEagerExprData::NewList {
                ref items,
                element_ty,
            } => VmirExprData::Linkage,
            HirEagerExprData::Block { stmts } => {
                VmirExprData::Block {
                    stmts: stmts.to_vmir(builder),
                    // ad hoc, todo: find destroyers
                    destroyers: builder.alloc_destroyers(vec![]),
                }
            }
            HirEagerExprData::Closure {
                ref parameters,
                return_ty,
                body,
            } => VmirExprData::Closure,
            HirEagerExprData::EmptyHtmlTag {
                function_ident,
                ref arguments,
            } => VmirExprData::Linkage,
            HirEagerExprData::Todo => VmirExprData::Todo,
            HirEagerExprData::Unreachable => VmirExprData::Unreachable,
        };
        builder.alloc_expr(expr_data)
    }
}
