use crate::*;
use husky_devsoul::{
    devsoul::IsDevsoul,
    helpers::{DevsoulException, DevsoulValue},
};
use husky_devsoul_interface::ki_repr::KiArgumentReprInterface;
use husky_devsoul_interface::{ki_control_flow::KiControlFlow, IsLinketImpl};
use husky_hir_opr::binary::HirBinaryOpr;
use husky_ki::{KiOpn, KiPatternData};
use husky_ki_repr::repr::{KiArgumentRepr, KiDomainRepr, KiRepr};
use husky_linket_impl::standard::StandardLinketImpl;
use husky_opr::{BinaryClosedOpr, BinaryComparisonOpr};
use husky_standard_devsoul::StandardDevsoul;
use husky_term_prelude::literal::Literal;
use husky_value_interface::IsValue;

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn eval_ki_domain_repr(
        &self,
        ki_domain_repr: KiDomainRepr,
    ) -> KiControlFlow<(), Infallible, DevsoulException<Devsoul>> {
        let db = self.db();
        let ki_domain = ki_domain_repr.ki_domain(db);
        let Some(var_deps) = ki_domain_repr.var_deps(db) else {
            match ki_domain_repr {
                KiDomainRepr::Omni => (),
                KiDomainRepr::ConditionSatisfied(_) => unreachable!(),
                KiDomainRepr::ConditionNotSatisfied(_) => unreachable!(),
                KiDomainRepr::StmtNotReturned(_) => unreachable!(),
                KiDomainRepr::ExprNotReturned(_) => unreachable!(),
            }
            return KiControlFlow::Continue(());
        };
        self.storage.get_or_try_init_ki_domain_value(
            ki_domain,
            var_deps
                .iter()
                .map(|&path| ((*path).into(), self.get_static_var_id(path)))
                .collect(),
            || self.eval_ki_domain_repr_aux(ki_domain_repr),
            db,
        )
    }

    pub fn eval_ki_domain_repr_aux(
        &self,
        ki_domain_repr: KiDomainRepr,
    ) -> KiControlFlow<(), Infallible, DevsoulException<Devsoul>> {
        match ki_domain_repr {
            KiDomainRepr::Omni => KiControlFlow::Continue(()),
            KiDomainRepr::ConditionSatisfied(condition_ki_repr) => {
                match self.eval_ki_repr(condition_ki_repr) {
                    KiControlFlow::Continue(value) => match value.to_bool() {
                        true => KiControlFlow::Continue(()),
                        false => KiControlFlow::Undefined,
                    },
                    KiControlFlow::LoopContinue => todo!(),
                    KiControlFlow::LoopExit(_) => todo!(),
                    KiControlFlow::Return(_) => todo!(),
                    KiControlFlow::Undefined => KiControlFlow::Undefined,
                    KiControlFlow::Throw(_) => todo!(),
                }
            }
            KiDomainRepr::ConditionNotSatisfied(condition_ki_repr) => {
                match self.eval_ki_repr(condition_ki_repr) {
                    KiControlFlow::Continue(value) => match value.to_bool() {
                        true => KiControlFlow::Undefined,
                        false => KiControlFlow::Continue(()),
                    },
                    KiControlFlow::LoopContinue => todo!(),
                    KiControlFlow::LoopExit(_) => todo!(),
                    KiControlFlow::Return(_) => todo!(),
                    KiControlFlow::Undefined => KiControlFlow::Undefined,
                    KiControlFlow::Throw(_) => todo!(),
                }
            }
            KiDomainRepr::StmtNotReturned(stmt_ki_repr) => match self.eval_ki_repr(stmt_ki_repr) {
                KiControlFlow::Continue(_) => KiControlFlow::Continue(()),
                KiControlFlow::LoopContinue => todo!(),
                KiControlFlow::LoopExit(_) => todo!(),
                KiControlFlow::Return(_) | KiControlFlow::Undefined => KiControlFlow::Undefined,
                KiControlFlow::Throw(_) => todo!(),
            },
            KiDomainRepr::ExprNotReturned(_) => todo!(),
        }
    }

    pub fn eval_ki_repr(&self, ki_repr: KiRepr) -> DevsoulKiControlFlow<Devsoul> {
        let db = self.comptime.db();
        if self.config.needs_caching(ki_repr.caching_class(db)) {
            let ki = ki_repr.ki(db);
            let var_deps = ki_repr.var_deps(db);
            self.get_or_try_init_ki_value(ki, var_deps, || self.eval_ki_repr_aux(ki_repr))
        } else {
            self.eval_ki_repr_aux(ki_repr)
        }
    }

    fn eval_ki_repr_aux(&self, ki_repr: KiRepr) -> DevsoulKiControlFlow<Devsoul> {
        let db = self.db();
        let ctx = self.dev_eval_context();
        let ki_domain_repr = ki_repr.ki_domain_repr(db);
        let result: DevsoulKiControlFlow<Devsoul> = match ki_repr.opn(db) {
            KiOpn::Return => todo!(),
            KiOpn::Require => {
                let arguments: &[_] = ki_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 2);
                let KiArgumentRepr::Simple(condition) = arguments[0] else {
                    unreachable!()
                };
                if self.eval_ki_repr(condition)?.to_bool() {
                    KiControlFlow::Continue(().into())
                } else {
                    let KiArgumentRepr::Simple(default) = arguments[1] else {
                        unreachable!()
                    };
                    KiControlFlow::Return(self.eval_ki_repr(default)?)
                }
            }
            KiOpn::Assert => {
                let arguments: &[_] = ki_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 1);
                let KiArgumentRepr::Simple(condition) = arguments[0] else {
                    unreachable!()
                };
                if !self.eval_ki_repr(condition)?.to_bool() {
                    todo!()
                }
                KiControlFlow::Continue(().into())
            }
            KiOpn::Literal(lit) => {
                // ad hoc
                let db = self.db();
                let value: DevsoulValue<Devsoul> = match lit {
                    Literal::Unit(_) => ().into(),
                    Literal::Bool(b) => b.into(),
                    Literal::I8(i) => i.into(),
                    Literal::I16(i) => i.into(),
                    Literal::I32(i) => i.into(),
                    Literal::I64(lit) => lit.value(db).into(),
                    Literal::I128(lit) => lit.value(db).into(),
                    Literal::ISize(lit) => (lit.value(db) as isize).into(),
                    Literal::U8(i) => i.into(),
                    Literal::U16(i) => i.into(),
                    Literal::U32(i) => i.into(),
                    Literal::U64(lit) => lit.value(db).into(),
                    Literal::U128(lit) => lit.value(db).into(),
                    Literal::USize(lit) => (lit.value(db) as usize).into(),
                    Literal::R8(_) => todo!(),
                    Literal::R16(_) => todo!(),
                    Literal::R32(_) => todo!(),
                    Literal::R64(_) => todo!(),
                    Literal::R128(_) => todo!(),
                    Literal::RSize(_) => todo!(),
                    Literal::Nat(_) => todo!(),
                    Literal::F32(lit) => lit.value(db).into_inner().into(),
                    Literal::F64(lit) => lit.value(db).into_inner().into(),
                    Literal::String(_) => todo!(),
                    Literal::StaticLifetime => todo!(),
                };
                KiControlFlow::Continue(value)
            }
            KiOpn::Val(_path) => {
                let expansion = ki_repr.expansion(db).unwrap();
                self.eval_root_stmts(expansion.root_hir_lazy_stmt_ki_reprs(db))
            }
            KiOpn::Linket(linket) => {
                let linket_impl = self.comptime.linket_impl(linket);
                let control_flow = linket_impl.eval_ki(
                    ki_repr.into(),
                    ki_domain_repr.into(),
                    unsafe {
                        std::mem::transmute::<_, &[KiArgumentReprInterface]>(
                            ki_repr.arguments(db) as &[KiArgumentRepr]
                        )
                    },
                    ctx,
                );
                control_flow
            }
            KiOpn::FunctionRitchie(_) => todo!(),
            KiOpn::Prefix(_) => todo!(),
            KiOpn::Suffix(_) => todo!(),
            KiOpn::Binary(opr) => {
                let arguments: &[_] = ki_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 2);
                let KiArgumentRepr::Simple(lopd) = arguments[0] else {
                    unreachable!()
                };
                let KiArgumentRepr::Simple(ropd) = arguments[1] else {
                    unreachable!()
                };
                match opr {
                    HirBinaryOpr::Closed(opr) => {
                        let lopd = self.eval_ki_repr(lopd)?;
                        let ropd = self.eval_ki_repr(ropd)?;
                        KiControlFlow::Continue(
                            match opr {
                                BinaryClosedOpr::Add => lopd + ropd,
                                BinaryClosedOpr::BitAnd => lopd & ropd,
                                BinaryClosedOpr::BitOr => lopd | ropd,
                                BinaryClosedOpr::BitXor => lopd ^ ropd,
                                BinaryClosedOpr::Div => lopd / ropd,
                                BinaryClosedOpr::Mul => lopd * ropd,
                                BinaryClosedOpr::RemEuclid => todo!(),
                                BinaryClosedOpr::Power => todo!(),
                                BinaryClosedOpr::Sub => lopd - ropd,
                            }
                            .into(),
                        )
                    }
                    HirBinaryOpr::Shift(_) => todo!(),
                    HirBinaryOpr::Assign => todo!(),
                    HirBinaryOpr::AssignClosed(_) => todo!(),
                    HirBinaryOpr::AssignShift(_) => todo!(),
                    HirBinaryOpr::Comparison(opr) => {
                        let lopd = self.eval_ki_repr(lopd)?;
                        let ropd = self.eval_ki_repr(ropd)?;
                        KiControlFlow::Continue(
                            match opr {
                                BinaryComparisonOpr::Eq => lopd == ropd,
                                BinaryComparisonOpr::Neq => lopd != ropd,
                                BinaryComparisonOpr::Geq => lopd >= ropd,
                                BinaryComparisonOpr::Greater => lopd > ropd,
                                BinaryComparisonOpr::Leq => lopd <= ropd,
                                BinaryComparisonOpr::Less => lopd < ropd,
                            }
                            .into(),
                        )
                    }
                    HirBinaryOpr::ShortCircuitLogic(_) => todo!(),
                }
            }
            KiOpn::EvalDiscarded => todo!(),
            KiOpn::Branches => {
                for ki_argument_repr in ki_repr.arguments(db) {
                    let KiArgumentRepr::Branch {
                        condition,
                        ref stmts,
                    } = *ki_argument_repr
                    else {
                        unreachable!()
                    };
                    if let Some(condition) = condition {
                        if !self.eval_ki_repr(condition)?.to_bool() {
                            continue;
                        }
                    }
                    return self.eval_stmts(stmts);
                }
                KiControlFlow::Continue(().into())
            }
            KiOpn::TypeVariant(path) => {
                let presenter = self
                    .comptime
                    .linket_impl(Linket::new_enum_index_presenter(
                        path.parent_ty_path(db),
                        db,
                    ))
                    .enum_index_value_presenter();
                KiControlFlow::Continue(DevsoulValue::<Devsoul>::from_enum_index(
                    path.index(db).raw(), // ad hoc
                    presenter,
                ))
            }
            KiOpn::Be { pattern_data } => {
                let arguments: &[_] = ki_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 1);
                let KiArgumentRepr::Simple(src) = arguments[0] else {
                    unreachable!()
                };
                let src = self.eval_ki_repr(src)?;
                KiControlFlow::Continue(
                    match pattern_data {
                        KiPatternData::None => src.is_none(),
                        KiPatternData::Some => src.is_some(),
                    }
                    .into(),
                )
            }
            KiOpn::Unwrap {} => {
                let arguments: &[_] = ki_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 1);
                let KiArgumentRepr::Simple(self_argument) = arguments[0] else {
                    unreachable!()
                };
                let self_argument = self.eval_ki_repr(self_argument)?;
                use ::husky_print_utils::p;
                use ::salsa::DebugWithDb;
                // ad hoc, todo: consider null case
                p!(ki_repr.source(db).debug_info(db));
                KiControlFlow::Continue(self_argument.unwrap())
            }
            KiOpn::Index => {
                // ad hoc
                let arguments: &[_] = ki_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 2);
                let KiArgumentRepr::Simple(self_argument) = arguments[0] else {
                    unreachable!()
                };
                let self_argument = self.eval_ki_repr(self_argument)?;
                let KiArgumentRepr::Simple(index) = arguments[1] else {
                    unreachable!()
                };
                let index = self.eval_ki_repr(index)?.to_usize();
                KiControlFlow::Continue(self_argument.index(index))
            }
        };
        result
    }

    fn eval_root_stmts(&self, stmt_ki_reprs: &[KiRepr]) -> DevsoulKiControlFlow<Devsoul> {
        match self.eval_stmts(stmt_ki_reprs) {
            KiControlFlow::Continue(value) | KiControlFlow::Return(value) => {
                KiControlFlow::Continue(value)
            }
            KiControlFlow::LoopContinue => unreachable!(),
            KiControlFlow::LoopExit(_) => unreachable!(),
            KiControlFlow::Undefined => unreachable!(),
            KiControlFlow::Throw(e) => KiControlFlow::Throw(e),
        }
    }

    fn eval_stmts(
        &self,
        stmt_ki_reprs: &[KiRepr],
    ) -> KiControlFlow<DevsoulValue<Devsoul>, DevsoulValue<Devsoul>, DevsoulException<Devsoul>>
    {
        for &stmt_ki_repr in &stmt_ki_reprs[..stmt_ki_reprs.len() - 1] {
            let _: () = self.eval_ki_repr(stmt_ki_repr)?.into();
        }
        self.eval_ki_repr(*stmt_ki_reprs.last().unwrap())
    }

    fn eval_ki_argument(
        &self,
        ki_argument_repr: &KiArgumentRepr,
    ) -> KiControlFlow<DevsoulValue<Devsoul>, DevsoulValue<Devsoul>, DevsoulException<Devsoul>>
    {
        match *ki_argument_repr {
            KiArgumentRepr::Simple(ki_repr) => self.eval_ki_repr(ki_repr),
            KiArgumentRepr::Keyed(_) => todo!(),
            KiArgumentRepr::Variadic(_) => todo!(),
            KiArgumentRepr::Branch {
                condition: _,
                stmts: _,
            } => todo!(),
            KiArgumentRepr::RuntimeConstants(_) => todo!(),
        }
    }
}

#[test]
fn ki_repr_eval_works() {
    use husky_entity_kind::MajorFormKind;
    use husky_entity_path::path::{major_item::MajorItemPath, ItemPath};
    use husky_entity_tree::helpers::paths::module_item_paths;
    use husky_path_utils::dev_paths::*;

    let dev_paths = HuskyLangDevPaths::new();
    let runtime: Pin<Box<DevRuntime<StandardDevsoul<()>>>> =
        DevRuntime::new(dev_paths.dev_root().join("examples/mnist-classifier"), None).unwrap();
    let db = runtime.db();
    let DevComptimeTarget::SingleCrate(crate_path) = runtime.comptime_target() else {
        unreachable!()
    };
    for &item_path in module_item_paths(db, crate_path.root_module_path(db)) {
        let ItemPath::MajorItem(MajorItemPath::Form(form_path)) = item_path else {
            continue;
        };
        if form_path.kind(db) != MajorFormKind::Val {
            continue;
        }
        let ki_repr = KiRepr::new_val(form_path, db);
        for path in ki_repr.var_deps(db) {
            let ItemPath::MajorItem(MajorItemPath::Form(path)) = path else {
                todo!()
            };
            let StandardLinketImpl::StaticVar {
                set_up_for_testing, ..
            } = runtime
                .comptime
                .linket_impl(Linket::new_static_var(path, db))
            else {
                unreachable!()
            };
            set_up_for_testing(0)
        }
        runtime.eval_ki_repr(ki_repr);
    }
}
