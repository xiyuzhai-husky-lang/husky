use self::coercion::VmirCoercion;
use crate::{
    destroyer::VmirDestroyerIdxRange, eval::EvalVmir, pattern::VmirPattern, stmt::VmirStmtIdxRange,
    *,
};
use husky_devsoul_interface::{
    vm_control_flow::{LinketImplVmControlFlow, VmControlFlow},
    VmArgumentValue,
};
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprIdx, HirEagerRitchieArgument};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_lifetime_utils::capture::Captures;
use husky_linket::{linket::Linket, template_argument::qual::LinQual};
use husky_literal_value::LiteralValue;
use husky_opr::{BinaryClosedOpr, BinaryShiftOpr};
use husky_place::place::{idx::PlaceIdx, EthPlace};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use smallvec::{smallvec, SmallVec};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirExprData<LinketImpl: IsLinketImpl> {
    Literal {
        value: LiteralValue,
    },
    Variable {
        place_idx: PlaceIdx,
        qual: LinQual,
    },
    Binary {
        lopd: VmirExprIdx<LinketImpl>,
        opr: HirBinaryOpr,
        ropd: VmirExprIdx<LinketImpl>,
    },
    Be {
        opd: VmirExprIdx<LinketImpl>,
        pattern: VmirPattern<LinketImpl>,
    },
    Prefix {
        opr: HirPrefixOpr,
        opd: VmirExprIdx<LinketImpl>,
    },
    Suffix {
        opd: VmirExprIdx<LinketImpl>,
        opr: HirSuffixOpr,
    },
    Unveil {
        linket_impl: LinketImpl,
        opd: VmirExprIdx<LinketImpl>,
    },
    Linket {
        linket_impl: LinketImpl,
        arguments: VmirArguments<LinketImpl>,
    },
    Block {
        stmts: VmirStmtIdxRange<LinketImpl>,
        destroyers: VmirDestroyerIdxRange,
    },
    Closure,
    Todo,
    Unreachable,
    As {
        opd: VmirExprIdx<LinketImpl>,
    },
    Index,
    PrincipalEntityPath,
    Unwrap {
        opd: VmirExprIdx<LinketImpl>,
    },
    ConstTemplateVariable,
}

pub type VmirExprArena<LinketImpl> = Arena<VmirExprData<LinketImpl>>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirExprIdx<LinketImpl: IsLinketImpl>(ArenaIdx<VmirExprData<LinketImpl>>);

impl<LinketImpl: IsLinketImpl> std::ops::Deref for VmirExprIdx<LinketImpl> {
    type Target = ArenaIdx<VmirExprData<LinketImpl>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirExprIdxRange<LinketImpl: IsLinketImpl>(ArenaIdxRange<VmirExprData<LinketImpl>>);

impl<'db, Linktime: IsLinktime> VmirBuilder<'db, Linktime> {
    pub(crate) fn alloc_exprs(
        &mut self,
        exprs: Vec<VmirExprData<Linktime::LinketImpl>>,
    ) -> VmirExprIdxRange<Linktime::LinketImpl> {
        VmirExprIdxRange(self.alloc_exprs_aux(exprs))
    }
}

impl<LinketImpl: IsLinketImpl> IntoIterator for VmirExprIdxRange<LinketImpl> {
    type Item = VmirExprIdx<LinketImpl>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(VmirExprIdx)
    }
}

impl<LinketImpl: IsLinketImpl> IntoIterator for &VmirExprIdxRange<LinketImpl> {
    type Item = VmirExprIdx<LinketImpl>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(VmirExprIdx)
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VmirArgument<LinketImpl: IsLinketImpl> {
    SelfValue {
        expr: VmirExprIdx<LinketImpl>,
    },
    Simple {
        expr: VmirExprIdx<LinketImpl>,
        coercion: VmirCoercion,
    },
    Variadic {
        exprs: VmirExprIdxRange<LinketImpl>,
    },
}
pub type VmirArguments<LinketImpl> = SmallVec<[VmirArgument<LinketImpl>; 4]>;

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for HirEagerExprIdx {
    type Output = VmirExprIdx<LinketImpl>;

    fn to_vmir<Linktime: IsLinktime<LinketImpl = LinketImpl>>(
        self,
        builder: &mut VmirBuilder<Linktime>,
    ) -> Self::Output {
        let expr_data = builder.build_vmir_expr(self);
        VmirExprIdx(builder.alloc_expr(expr_data))
    }
}

impl<'comptime, Linktime: IsLinktime> VmirBuilder<'comptime, Linktime> {
    fn build_vmir_expr(&mut self, expr: HirEagerExprIdx) -> VmirExprData<Linktime::LinketImpl> {
        let entry = &self.hir_eager_expr_arena()[expr];
        match *entry.data() {
            HirEagerExprData::Literal(lit) => VmirExprData::Literal {
                value: lit.into_literal_value(self.db()),
            },
            HirEagerExprData::PrincipalEntityPath(_) => VmirExprData::PrincipalEntityPath,
            HirEagerExprData::AssocRitchie { assoc_item_path } => todo!(),
            HirEagerExprData::ComptimeVariable { ident } => VmirExprData::ConstTemplateVariable,
            HirEagerExprData::RuntimeVariable(_) => {
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
                linket_impl: self.linket_impl(Linket::new_unveil_assoc_fn(
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
                let linket = Linket::new_ty_constructor_fn(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linket_impl = self.linket_impl(linket);
                let arguments = self.build_arguments(item_groups).collect();
                VmirExprData::Linket {
                    linket_impl,
                    arguments,
                }
            }
            HirEagerExprData::TypeVariantConstructorCall {
                path,
                ref instantiation,
                ref arguments,
            } => {
                let linket = Linket::new_ty_variant_constructor_fn(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linket_impl = self.linket_impl(linket);
                let arguments = self.build_arguments(arguments).collect();
                VmirExprData::Linket {
                    linket_impl,
                    arguments,
                }
            }
            HirEagerExprData::FunctionRitchieCall {
                path,
                ref instantiation,
                ref arguments,
            } => {
                let linket = Linket::new_major_function_ritchie_item(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linket_impl = self.linket_impl(linket);
                let arguments = self.build_arguments(arguments).collect();
                VmirExprData::Linket {
                    linket_impl,
                    arguments,
                }
            }
            HirEagerExprData::AssocFunctionRitchieCall {
                path,
                ref instantiation,
                ref arguments,
            } => {
                let linket = Linket::new_assoc_function_ritchie_item(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linket_impl = self.linket_impl(linket);
                let arguments = self.build_arguments(arguments).collect();
                VmirExprData::Linket {
                    linket_impl,
                    arguments,
                }
            }
            HirEagerExprData::PropsStructField {
                self_argument,
                self_ty,
                ident,
                ..
            } => {
                let linket = Linket::new_props_struct_field(
                    self_ty,
                    ident,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linket_impl = self.linket_impl(linket);
                let arguments = smallvec![VmirArgument::SelfValue {
                    expr: self_argument.to_vmir(self)
                }];
                VmirExprData::Linket {
                    linket_impl,
                    arguments,
                }
            }
            HirEagerExprData::MemoizedField {
                self_argument,
                self_argument_ty,
                self_ty,
                ident,
                path,
                ref indirections,
                ref instantiation,
            } => {
                let linket = Linket::new_memo_field(
                    path,
                    instantiation,
                    self.lin_instantiation(),
                    self.db(),
                );
                let linket_impl = self.linket_impl(linket);
                let arguments = smallvec![VmirArgument::SelfValue {
                    expr: self_argument.to_vmir(self)
                }];
                if indirections.len() > 0 {
                    todo!()
                }
                VmirExprData::Linket {
                    linket_impl,
                    arguments,
                }
            }
            HirEagerExprData::MethodRitchieCall {
                self_argument,
                self_ty,
                self_contract,
                ident,
                path,
                ref indirections,
                ref instantiation,
                arguments: ref hir_arguments,
            } => {
                let linket =
                    Linket::new_method(path, instantiation, self.lin_instantiation(), self.db());
                let linket_impl = self.linket_impl(linket);
                let mut arguments = smallvec![VmirArgument::SelfValue {
                    expr: self_argument.to_vmir(self)
                }];
                arguments.extend(self.build_arguments(hir_arguments));
                if indirections.len() > 0 {
                    todo!()
                }
                VmirExprData::Linket {
                    linket_impl,
                    arguments,
                }
            }
            HirEagerExprData::NewTuple { .. } => VmirExprData::Linket {
                linket_impl: todo!(),
                arguments: todo!(),
            },
            HirEagerExprData::Index {
                self_argument: owner,
                ref items,
            } => VmirExprData::Index,
            HirEagerExprData::NewList {
                ref exprs,
                element_ty,
            } => {
                let linket =
                    Linket::new_vec_constructor(element_ty, self.lin_instantiation(), self.db());
                let linket_impl = self.linket_impl(linket);
                let exprs = exprs
                    .iter()
                    .map(|&item| self.build_vmir_expr(item))
                    .collect();
                VmirExprData::Linket {
                    linket_impl,
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
                VmirExprData::Linket {
                    linket_impl: todo!(),
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
    ) -> impl Iterator<Item = VmirArgument<Linktime::LinketImpl>> + Captures<'comptime> + 'a {
        arguments.iter().map(move |m| match m {
            HirEagerRitchieArgument::Simple(_, arg, coercion) => VmirArgument::Simple {
                expr: arg.to_vmir(self),
                coercion: coercion.to_vmir(self),
            },
            HirEagerRitchieArgument::Variadic => {
                todo!()
            }
            HirEagerRitchieArgument::Keyed => todo!(),
        })
    }
}

impl<LinketImpl: IsLinketImpl> VmirExprIdx<LinketImpl> {
    #[inline(always)]
    pub fn eval<'comptime>(
        self,
        coercion: impl Into<Option<VmirCoercion>>,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl> {
        let value = ctx.eval_expr(self, |ctx| self.eval_aux(ctx))?;
        VmControlFlow::Continue(match coercion.into() {
            Some(coercion) => match coercion {
                VmirCoercion::Trivial => value,
                VmirCoercion::Never => todo!(),
                VmirCoercion::WrapInSome => todo!(),
                VmirCoercion::Redirection => todo!(),
                VmirCoercion::Dedirection => todo!(),
            },
            None => value,
        })
    }

    #[inline(always)]
    fn eval_aux<'comptime>(
        self,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) -> LinketImplVmControlFlow<LinketImpl> {
        use VmControlFlow::*;

        match *self.entry(ctx.vmir_expr_arena()) {
            VmirExprData::Literal { ref value } => Continue(value.into_value()),
            VmirExprData::Variable { place_idx, qual } => {
                Continue(ctx.access_place(place_idx, qual))
            }
            VmirExprData::Binary { lopd, opr, ropd } => {
                let lopd = lopd.eval(None, ctx)?;
                let ropd = ropd.eval(None, ctx)?;
                ctx.eval_expr_itself(self, |_ctx| match opr {
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
                ctx.eval_expr_itself(self, |_ctx| match opr {
                    HirPrefixOpr::Minus => Continue(-opd),
                    HirPrefixOpr::NotBool => todo!(),
                    HirPrefixOpr::NotInt => todo!(),
                    HirPrefixOpr::BitNot => todo!(),
                    HirPrefixOpr::TakeRef => todo!(),
                    HirPrefixOpr::Deref => todo!(),
                })
            }
            VmirExprData::Suffix { opd, opr } => todo!(),
            VmirExprData::Unveil { linket_impl, opd } => todo!(),
            VmirExprData::Linket {
                linket_impl,
                ref arguments,
            } => {
                let arguments = arguments
                    .iter()
                    .map(
                        |arg| -> LinketImplVmControlFlow<LinketImpl, VmArgumentValue<LinketImpl>> {
                            match arg {
                                VmirArgument::SelfValue { expr } => todo!(),
                                VmirArgument::Simple { expr, coercion } => todo!(),
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
                ctx.eval_expr_itself(self, |ctx| linket_impl.eval_vm(arguments, ctx.db()))
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
