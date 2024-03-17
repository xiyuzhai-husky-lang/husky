use crate::{destroyer::VmirDestroyerIdxRange, stmt::VmirStmtIdxRange, *};
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprIdx, HirEagerRitchieArgument};
use husky_lifetime_utils::capture::Captures;
use husky_linkage::linkage::Linkage;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use smallvec::SmallVec;

use self::coersion::VmirCoersion;

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
        arguments: VmirArguments<LinkageImpl>,
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

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirArgument<LinkageImpl: IsLinkageImpl> {
    Simple {
        expr: VmirExprIdx<LinkageImpl>,
        coersion: VmirCoersion,
    },
}
pub type VmirArguments<LinkageImpl> = SmallVec<[VmirArgument<LinkageImpl>; 4]>;

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
                let arguments = self.build_arguments(item_groups).collect();
                VmirExprData::Linkage {
                    linkage_impl,
                    arguments,
                }
            }
            HirEagerExprData::TypeVariantConstructorCall {
                path,
                ref instantiation,
                ref arguments,
            } => {
                let linkage = Linkage::new_ty_variant_constructor_fn(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linkage_impl = self.linkage_impl(linkage);
                let arguments = self.build_arguments(arguments).collect();
                VmirExprData::Linkage {
                    linkage_impl,
                    arguments,
                }
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
                    arguments: todo!(),
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
                    arguments: todo!(),
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
                    arguments: todo!(),
                }
            }
            HirEagerExprData::MemoizedField {
                owner_hir_expr_idx,
                ident,
                path,
            } => VmirExprData::Linkage {
                linkage_impl: todo!(),
                arguments: todo!(),
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
                arguments: todo!(),
            },
            HirEagerExprData::NewTuple { .. } => VmirExprData::Linkage {
                linkage_impl: todo!(),
                arguments: todo!(),
            },
            HirEagerExprData::Index { owner, ref items } => VmirExprData::Index,
            HirEagerExprData::NewList {
                ref items,
                element_ty,
            } => VmirExprData::Linkage {
                linkage_impl: todo!(),
                arguments: todo!(),
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
                let arguments = todo!();
                VmirExprData::Linkage {
                    linkage_impl: todo!(),
                    arguments,
                }
            }
            HirEagerExprData::Todo => VmirExprData::Todo,
            HirEagerExprData::Unreachable => VmirExprData::Unreachable,
        }
    }

    fn build_arguments<'a>(
        &'a mut self,
        item_groups: &'db [HirEagerRitchieArgument],
    ) -> impl Iterator<Item = VmirArgument<Linktime::LinkageImpl>> + Captures<&'db ()> + 'a {
        item_groups.iter().map(move |m| match m {
            HirEagerRitchieArgument::Simple(_, arg, coersion) => VmirArgument::Simple {
                expr: arg.to_vmir(self),
                coersion: coersion.to_vmir(self),
            },
            HirEagerRitchieArgument::Variadic => {
                todo!()
            }
            HirEagerRitchieArgument::Keyed => todo!(),
        })
    }
}
