use crate::{
    builder::VmirExprBuilder, destroyer::VmirDestroyerIdxRange, stmt::VmirStmtIdxRange, ToVmir,
};
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprIdx};
use husky_linkage::linkage::Linkage;
use husky_task_interface::IsLinkageImpl;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirExprData<LinkageImpl: IsLinkageImpl> {
    Literal,
    Variable,
    Binary,
    Be,
    Prefix,
    Suffix,
    Unveil,
    Linkage(LinkageImpl),
    Block {
        stmts: VmirStmtIdxRange<LinkageImpl>,
        destroyers: VmirDestroyerIdxRange,
    },
    Closure,
    Todo,
    Unreachable,
}

pub type VmirExprArena<LinkageImpl> = Arena<VmirExprData<LinkageImpl>>;
pub type VmirExprIdx<LinkageImpl> = ArenaIdx<VmirExprData<LinkageImpl>>;
pub type VmirExprIdxRange<LinkageImpl> = ArenaIdxRange<VmirExprData<LinkageImpl>>;

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerExprIdx {
    type Output = VmirExprIdx<LinkageImpl>;

    fn to_vmir(self, builder: &mut VmirExprBuilder<LinkageImpl>) -> Self::Output {
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
            } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::TypeVariantConstructorCall {
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::FunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::AssocFunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::PropsStructField {
                owner,
                ident,
                field_ty,
            } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::MemoizedField {
                owner_hir_expr_idx,
                ident,
                path,
            } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::MethodFnCall {
                self_argument,
                self_contract,
                ident,
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::NewTuple { .. } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::Index { owner, ref items } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::NewList {
                ref items,
                element_ty,
            } => VmirExprData::Linkage(todo!()),
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
            } => VmirExprData::Linkage(todo!()),
            HirEagerExprData::Todo => VmirExprData::Todo,
            HirEagerExprData::Unreachable => VmirExprData::Unreachable,
        };
        builder.alloc_expr(expr_data)
    }
}
