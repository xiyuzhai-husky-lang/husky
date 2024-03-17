use crate::{destroyer::VmirDestroyerIdxRange, stmt::VmirStmtIdxRange, *};
use husky_hir_eager_expr::{
    HirEagerExprData, HirEagerExprIdx, HirEagerRitchieParameterArgumentMatch,
};
use husky_lifetime_utils::capture::Captures;
use husky_linkage::linkage::Linkage;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirExprData<LinkageImpl: IsLinkageImpl> {
    Literal,
    Variable,
    Binary,
    Be,
    Prefix,
    Suffix,
    Unveil,
    Linkage {
        linkage_impl: LinkageImpl,
        opds: VmirExprIdxRange<LinkageImpl>,
    },
    Block {
        stmts: VmirStmtIdxRange<LinkageImpl>,
        destroyers: VmirDestroyerIdxRange,
    },
    Closure,
    Todo,
    Unreachable,
    As,
    Index,
}

pub type VmirExprArena<LinkageImpl> = Arena<VmirExprData<LinkageImpl>>;
pub type VmirExprIdx<LinkageImpl> = ArenaIdx<VmirExprData<LinkageImpl>>;
pub type VmirExprIdxRange<LinkageImpl> = ArenaIdxRange<VmirExprData<LinkageImpl>>;

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerExprIdx {
    type Output = VmirExprIdx<LinkageImpl>;

    fn to_vmir<Linktime: IsLinktime<LinkageImpl = LinkageImpl>>(
        self,
        builder: &mut VmirExprBuilder<Linktime>,
    ) -> Self::Output {
        let expr_data = builder.build_vmir_expr(self);
        builder.alloc_expr(expr_data)
    }
}

impl<'db, Linktime: IsLinktime> VmirExprBuilder<'db, Linktime> {
    fn build_vmir_expr(&mut self, expr: HirEagerExprIdx) -> VmirExprData<Linktime::LinkageImpl> {
        match *self.hir_eager_expr_arena()[expr].data() {
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
            HirEagerExprData::As { opd, ty } => VmirExprData::As,
            HirEagerExprData::TypeConstructorFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => {
                let linkage = Linkage::new_ty_constructor_fn(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linkage_impl = self.linkage_impl(linkage);
                let opds = self.build_item_groups(item_groups).collect();
                let opds = self.alloc_exprs(opds);
                VmirExprData::Linkage { linkage_impl, opds }
            }
            HirEagerExprData::TypeVariantConstructorCall {
                path,
                ref instantiation,
                ref item_groups,
            } => {
                let linkage = Linkage::new_ty_variant_constructor_fn(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linkage_impl = self.linkage_impl(linkage);
                let opds = self.build_item_groups(item_groups).collect();
                let opds = self.alloc_exprs(opds);
                VmirExprData::Linkage { linkage_impl, opds }
            }
            HirEagerExprData::FunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => {
                let linkage = Linkage::new_function_fn_item(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                VmirExprData::Linkage {
                    linkage_impl: self.linkage_impl(linkage),
                    opds: todo!(),
                }
            }
            HirEagerExprData::AssocFunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => {
                let linkage = Linkage::new_assoc_function_fn_item(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                VmirExprData::Linkage {
                    linkage_impl: self.linkage_impl(linkage),
                    opds: todo!(),
                }
            }
            HirEagerExprData::PropsStructField {
                owner_base_ty,
                ident,
                ..
            } => {
                let linkage = Linkage::new_props_struct_field(
                    owner_base_ty,
                    ident,
                    self.lin_instantiation(),
                    self.db(),
                );
                VmirExprData::Linkage {
                    linkage_impl: self.linkage_impl(linkage),
                    opds: todo!(),
                }
            }
            HirEagerExprData::MemoizedField {
                owner_hir_expr_idx,
                ident,
                path,
            } => VmirExprData::Linkage {
                linkage_impl: todo!(),
                opds: todo!(),
            },
            HirEagerExprData::MethodFnCall {
                self_argument,
                self_contract,
                ident,
                path,
                ref instantiation,
                ref item_groups,
            } => VmirExprData::Linkage {
                linkage_impl: todo!(),
                opds: todo!(),
            },
            HirEagerExprData::NewTuple { .. } => VmirExprData::Linkage {
                linkage_impl: todo!(),
                opds: todo!(),
            },
            HirEagerExprData::Index { owner, ref items } => VmirExprData::Index,
            HirEagerExprData::NewList {
                ref items,
                element_ty,
            } => VmirExprData::Linkage {
                linkage_impl: todo!(),
                opds: todo!(),
            },
            HirEagerExprData::Block { stmts } => {
                VmirExprData::Block {
                    stmts: stmts.to_vmir(self),
                    // ad hoc, todo: find destroyers
                    destroyers: self.alloc_destroyers(vec![]),
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
            } => {
                let opds = todo!();
                VmirExprData::Linkage {
                    linkage_impl: todo!(),
                    opds,
                }
            }
            HirEagerExprData::Todo => VmirExprData::Todo,
            HirEagerExprData::Unreachable => VmirExprData::Unreachable,
        }
    }

    fn build_item_groups<'a>(
        &'a mut self,
        item_groups: &'db [HirEagerRitchieParameterArgumentMatch],
    ) -> impl Iterator<Item = VmirExprData<Linktime::LinkageImpl>> + Captures<&'db ()> + 'a {
        item_groups.iter().map(move |m| match m {
            HirEagerRitchieParameterArgumentMatch::Regular(_, arg, _) => self.build_vmir_expr(*arg),
            HirEagerRitchieParameterArgumentMatch::Variadic => {
                todo!()
            }
            HirEagerRitchieParameterArgumentMatch::Keyed => todo!(),
        })
    }
}
