use self::coersion::VmirCoersion;
use crate::{
    destroyer::VmirDestroyerIdxRange, eval::EvalVmir, pattern::VmirPattern, stmt::VmirStmtIdxRange,
    *,
};
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprIdx, HirEagerRitchieArgument};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_lifetime_utils::capture::Captures;
use husky_linkage::{linkage::Linkage, template_argument::qual::LinQual};
use husky_literal_value::LiteralValue;
use husky_opr::{BinaryClosedOpr, BinaryShiftOpr};
use husky_place::place::{idx::PlaceIdx, EthPlace};
use husky_task_interface::{
    vm_control_flow::{LinkageImplVmControlFlow, VmControlFlow},
    VmArgumentValue,
};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use smallvec::{smallvec, SmallVec};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirExprData<LinkageImpl: IsLinkageImpl> {
    Literal {
        value: LiteralValue,
    },
    Variable {
        place_idx: PlaceIdx,
        qual: LinQual,
    },
    Binary {
        lopd: VmirExprIdx<LinkageImpl>,
        opr: HirBinaryOpr,
        ropd: VmirExprIdx<LinkageImpl>,
    },
    Be {
        opd: VmirExprIdx<LinkageImpl>,
        pattern: VmirPattern<LinkageImpl>,
    },
    Prefix {
        opr: HirPrefixOpr,
        opd: VmirExprIdx<LinkageImpl>,
    },
    Suffix {
        opd: VmirExprIdx<LinkageImpl>,
        opr: HirSuffixOpr,
    },
    Unveil {
        linkage_impl: LinkageImpl,
        opd: VmirExprIdx<LinkageImpl>,
    },
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
    As {
        opd: VmirExprIdx<LinkageImpl>,
    },
    Index,
    PrincipalEntityPath,
    Unwrap {
        opd: VmirExprIdx<LinkageImpl>,
    },
    ConstTemplateVariable,
}

pub type VmirExprArena<LinkageImpl> = Arena<VmirExprData<LinkageImpl>>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirExprIdx<LinkageImpl: IsLinkageImpl>(ArenaIdx<VmirExprData<LinkageImpl>>);

impl<LinkageImpl: IsLinkageImpl> std::ops::Deref for VmirExprIdx<LinkageImpl> {
    type Target = ArenaIdx<VmirExprData<LinkageImpl>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirExprIdxRange<LinkageImpl: IsLinkageImpl>(ArenaIdxRange<VmirExprData<LinkageImpl>>);

impl<'db, Linktime: IsLinktime> VmirBuilder<'db, Linktime> {
    pub(crate) fn alloc_exprs(
        &mut self,
        exprs: Vec<VmirExprData<Linktime::LinkageImpl>>,
    ) -> VmirExprIdxRange<Linktime::LinkageImpl> {
        VmirExprIdxRange(self.alloc_exprs_aux(exprs))
    }
}

impl<LinkageImpl: IsLinkageImpl> IntoIterator for VmirExprIdxRange<LinkageImpl> {
    type Item = VmirExprIdx<LinkageImpl>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(VmirExprIdx)
    }
}

impl<LinkageImpl: IsLinkageImpl> IntoIterator for &VmirExprIdxRange<LinkageImpl> {
    type Item = VmirExprIdx<LinkageImpl>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(VmirExprIdx)
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirArgument<LinkageImpl: IsLinkageImpl> {
    SelfValue {
        expr: VmirExprIdx<LinkageImpl>,
    },
    Simple {
        expr: VmirExprIdx<LinkageImpl>,
        coersion: VmirCoersion,
    },
    Variadic {
        exprs: VmirExprIdxRange<LinkageImpl>,
    },
}
pub type VmirArguments<LinkageImpl> = SmallVec<[VmirArgument<LinkageImpl>; 4]>;

impl<LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for HirEagerExprIdx {
    type Output = VmirExprIdx<LinkageImpl>;

    fn to_vmir<Linktime: IsLinktime<LinkageImpl = LinkageImpl>>(
        self,
        builder: &mut VmirBuilder<Linktime>,
    ) -> Self::Output {
        let expr_data = builder.build_vmir_expr(self);
        VmirExprIdx(builder.alloc_expr(expr_data))
    }
}

impl<'comptime, Linktime: IsLinktime> VmirBuilder<'comptime, Linktime> {
    fn build_vmir_expr(&mut self, expr: HirEagerExprIdx) -> VmirExprData<Linktime::LinkageImpl> {
        let entry = &self.hir_eager_expr_arena()[expr];
        match *entry.data() {
            HirEagerExprData::Literal(lit) => VmirExprData::Literal {
                value: lit.into_literal_value(self.db()),
            },
            HirEagerExprData::PrincipalEntityPath(_) => VmirExprData::PrincipalEntityPath,
            HirEagerExprData::AssocRitchie { assoc_item_path } => todo!(),
            HirEagerExprData::ConstVariable { ident } => VmirExprData::ConstTemplateVariable,
            HirEagerExprData::Variable(_) => {
                let place_idx = match entry.quary().place() {
                    Some(place) => match place {
                        EthPlace::Idx(place_idx) => place_idx,
                        EthPlace::SymbolicVariable(_) => todo!(),
                        EthPlace::Field(_) => todo!(),
                    },
                    None => {
                        use husky_print_utils::p;

                        p!(entry.contracted_quary());
                        todo!()
                    }
                };
                let qual = LinQual::from_hir(entry.contracted_quary());
                VmirExprData::Variable { place_idx, qual }
            }
            HirEagerExprData::Binary { lopd, opr, ropd } => VmirExprData::Binary {
                lopd: lopd.to_vmir(self),
                opr,
                ropd: ropd.to_vmir(self),
            },
            HirEagerExprData::Be { src, ref pattern } => VmirExprData::Be {
                opd: src.to_vmir(self),
                pattern: pattern.pattern.to_vmir(self),
            },
            HirEagerExprData::Prefix { opr, opd } => VmirExprData::Prefix {
                opr,
                opd: opd.to_vmir(self),
            },
            HirEagerExprData::Suffix { opd, opr } => VmirExprData::Suffix {
                opd: opd.to_vmir(self),
                opr,
            },
            HirEagerExprData::Unveil {
                unveil_assoc_fn_path,
                ref instantiation,
                opd,
                ..
            } => VmirExprData::Unveil {
                linkage_impl: self.linkage_impl(Linkage::new_unveil_assoc_fn(
                    unveil_assoc_fn_path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                )),
                opd: opd.to_vmir(self),
            },
            HirEagerExprData::Unwrap { opd } => VmirExprData::Unwrap {
                opd: opd.to_vmir(self),
            },
            HirEagerExprData::As { opd, ty } => VmirExprData::As {
                opd: opd.to_vmir(self),
            },
            HirEagerExprData::TypeConstructorCall {
                path,
                ref instantiation,
                arguments: ref item_groups,
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
            HirEagerExprData::FunctionRitchieCall {
                path,
                ref instantiation,
                ref arguments,
            } => {
                let linkage = Linkage::new_major_function_ritchie_item(
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
            HirEagerExprData::AssocFunctionRitchieCall {
                path,
                ref instantiation,
                ref arguments,
            } => {
                let linkage = Linkage::new_assoc_function_ritchie_item(
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
            HirEagerExprData::PropsStructField {
                self_argument,
                self_ty,
                ident,
                ..
            } => {
                let linkage = Linkage::new_props_struct_field(
                    self_ty,
                    ident,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linkage_impl = self.linkage_impl(linkage);
                let arguments = smallvec![VmirArgument::SelfValue {
                    expr: self_argument.to_vmir(self)
                }];
                VmirExprData::Linkage {
                    linkage_impl,
                    arguments,
                }
            }
            HirEagerExprData::MemoizedField {
                self_argument,
                self_ty,
                ident,
                path,
                ref instantiation,
            } => {
                let linkage = Linkage::new_memo_field(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linkage_impl = self.linkage_impl(linkage);
                let arguments = smallvec![VmirArgument::SelfValue {
                    expr: self_argument.to_vmir(self)
                }];
                VmirExprData::Linkage {
                    linkage_impl,
                    arguments,
                }
            }
            HirEagerExprData::MethodRitchieCall {
                self_argument,
                self_contract,
                ident,
                path,
                ref instantiation,
                arguments: ref hir_arguments,
            } => {
                let linkage =
                    Linkage::new_method(path, instantiation, self.lin_instantiation(), self.db());
                let linkage_impl = self.linkage_impl(linkage);
                let mut arguments = smallvec![VmirArgument::SelfValue {
                    expr: self_argument.to_vmir(self)
                }];
                arguments.extend(self.build_arguments(hir_arguments));
                VmirExprData::Linkage {
                    linkage_impl,
                    arguments,
                }
            }
            HirEagerExprData::NewTuple { .. } => VmirExprData::Linkage {
                linkage_impl: todo!(),
                arguments: todo!(),
            },
            HirEagerExprData::Index { owner, ref items } => VmirExprData::Index,
            HirEagerExprData::NewList {
                ref exprs,
                element_ty,
            } => {
                let linkage =
                    Linkage::new_vec_constructor(element_ty, self.lin_instantiation(), self.db());
                let linkage_impl = self.linkage_impl(linkage);
                let exprs = exprs
                    .iter()
                    .map(|&item| self.build_vmir_expr(item))
                    .collect();
                VmirExprData::Linkage {
                    linkage_impl,
                    arguments: smallvec![VmirArgument::Variadic {
                        exprs: self.alloc_exprs(exprs),
                    }],
                }
            }
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
        arguments: &'comptime [HirEagerRitchieArgument],
    ) -> impl Iterator<Item = VmirArgument<Linktime::LinkageImpl>> + Captures<&'comptime ()> + 'a
    {
        arguments.iter().map(move |m| match m {
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

impl<LinkageImpl: IsLinkageImpl> VmirExprIdx<LinkageImpl> {
    #[inline(always)]
    pub fn eval<'comptime>(
        self,
        coersion: impl Into<Option<VmirCoersion>>,
        ctx: &mut impl EvalVmir<'comptime, LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        let value = ctx.eval_expr(self, |ctx| self.eval_aux(ctx))?;
        VmControlFlow::Continue(match coersion.into() {
            Some(coersion) => match coersion {
                VmirCoersion::Trivial => value,
                VmirCoersion::Never => todo!(),
                VmirCoersion::WrapInSome => todo!(),
                VmirCoersion::PlaceToLeash => todo!(),
                VmirCoersion::Deref => todo!(),
            },
            None => value,
        })
    }

    #[inline(always)]
    fn eval_aux<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinkageImpl>,
    ) -> LinkageImplVmControlFlow<LinkageImpl> {
        use VmControlFlow::*;

        match *self.entry(ctx.vmir_expr_arena()) {
            VmirExprData::Literal { ref value } => Continue(value.into_value()),
            VmirExprData::Variable { place_idx, qual } => {
                Continue(ctx.access_place(place_idx, qual))
            }
            VmirExprData::Binary { lopd, opr, ropd } => {
                let lopd = lopd.eval(None, ctx)?;
                let ropd = ropd.eval(None, ctx)?;
                ctx.eval_expr_inner(self, |_ctx| match opr {
                    HirBinaryOpr::Closed(opr) => Continue(match opr {
                        BinaryClosedOpr::Add => lopd + ropd,
                        BinaryClosedOpr::BitAnd => lopd & ropd,
                        BinaryClosedOpr::BitOr => lopd | ropd,
                        BinaryClosedOpr::BitXor => lopd ^ ropd,
                        BinaryClosedOpr::Div => lopd / ropd,
                        BinaryClosedOpr::Mul => lopd * ropd,
                        BinaryClosedOpr::RemEuclid => todo!("be careful"),
                        BinaryClosedOpr::Power => todo!(),
                        BinaryClosedOpr::Sub => lopd - ropd,
                    }),
                    HirBinaryOpr::Shift(opr) => Continue(match opr {
                        BinaryShiftOpr::Shl => lopd << ropd,
                        BinaryShiftOpr::Shr => lopd >> ropd,
                    }),
                    HirBinaryOpr::Assign => todo!(),
                    HirBinaryOpr::AssignClosed(_) => todo!(),
                    HirBinaryOpr::AssignShift(_) => todo!(),
                    HirBinaryOpr::Comparison(_) => todo!(),
                    HirBinaryOpr::ShortCircuitLogic(_) => todo!(),
                })
            }
            VmirExprData::Be { opd, pattern } => todo!(),
            VmirExprData::Prefix { opr, opd } => {
                let opd = opd.eval(None, ctx)?;
                ctx.eval_expr_inner(self, |_ctx| match opr {
                    HirPrefixOpr::Minus => Continue(-opd),
                    HirPrefixOpr::NotBool => todo!(),
                    HirPrefixOpr::NotInt => todo!(),
                    HirPrefixOpr::BitNot => todo!(),
                    HirPrefixOpr::TakeRef => todo!(),
                    HirPrefixOpr::Deref => todo!(),
                })
            }
            VmirExprData::Suffix { opd, opr } => todo!(),
            VmirExprData::Unveil { linkage_impl, opd } => todo!(),
            VmirExprData::Linkage {
                linkage_impl,
                ref arguments,
            } => {
                let arguments =
                    arguments
                        .iter()
                        .map(
                            |arg| -> LinkageImplVmControlFlow<
                                LinkageImpl,
                                VmArgumentValue<LinkageImpl>,
                            > {
                                match arg {
                                    VmirArgument::SelfValue { expr } => todo!(),
                                    VmirArgument::Simple { expr, coersion } => todo!(),
                                    VmirArgument::Variadic { exprs } => {
                                        VmControlFlow::Continue(VmArgumentValue::Variadic(
                                            exprs
                                                .into_iter()
                                                .map(|expr| expr.eval(None, ctx))
                                                .collect::<VmControlFlow<_, _, _>>()?,
                                        ))
                                    }
                                }
                            },
                        )
                        .collect::<VmControlFlow<_, _, _>>()?;
                ctx.eval_expr_inner(self, |ctx| linkage_impl.eval_vm(arguments, ctx.db()))
            }
            VmirExprData::Block { stmts, destroyers } => stmts.eval(ctx),
            VmirExprData::Closure => todo!(),
            VmirExprData::Todo => todo!(),
            VmirExprData::Unreachable => todo!(),
            VmirExprData::As { opd } => todo!(),
            VmirExprData::Index => todo!(),
            VmirExprData::PrincipalEntityPath => todo!(),
            VmirExprData::Unwrap { opd } => todo!(),
            VmirExprData::ConstTemplateVariable => todo!(),
        }
    }
}
