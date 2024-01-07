use crate::*;
use husky_entity_path::{TypeVariantIndex};

use husky_fluffy_term::FluffyTermEngine;
use husky_hir_opr::binary::HirBinaryOpr;
use husky_opr::BinaryComparisonOpr;
use husky_task::{
    dev_ascension::{dev_eval_context, with_runtime_and_base_point},
    helpers::{TaskError, TaskValue},
    IsTask,
};
use husky_task_interface::value::IsValue;
use husky_task_interface::{val_control_flow::ValControlFlow, IsLinkageImpl};
use husky_term_prelude::TermLiteral;
use husky_val::{ValOpn, ValPatternData};
use husky_val_repr::repr::{ValArgumentRepr, ValRepr};





impl<Task: IsTask> DevRuntime<Task> {
    pub fn eval_val_repr_at_pedestal(
        &self,
        val_repr: ValRepr,
        pedestal: TaskDevPedestal<Task>,
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        with_runtime_and_base_point::<TaskDevAscension<Task>, _, _>(self, pedestal, || {
            self.eval_val_repr(val_repr)
        })
    }

    fn eval_val_repr(
        &self,
        val_repr: ValRepr,
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        // todo: consider domain
        let db = self.db();
        let result = match val_repr.opn(db) {
            ValOpn::Return => todo!(),
            ValOpn::Require => {
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 2);
                let ValArgumentRepr::Ordinary(condition) = arguments[0] else {
                    unreachable!()
                };
                if self.eval_val_repr(condition)?.to_bool() {
                    ValControlFlow::Continue(().into())
                } else {
                    let ValArgumentRepr::Ordinary(default) = arguments[1] else {
                        unreachable!()
                    };
                    ValControlFlow::Return(self.eval_val_repr(default)?)
                }
            }
            ValOpn::Assert => {
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 1);
                let ValArgumentRepr::Ordinary(condition) = arguments[0] else {
                    unreachable!()
                };
                if !self.eval_val_repr(condition)?.to_bool() {
                    todo!()
                }
                ValControlFlow::Continue(().into())
            }
            ValOpn::Literal(lit) => {
                // ad hoc
                let value: TaskValue<Task> = match lit {
                    TermLiteral::Unit(_) => todo!(),
                    TermLiteral::Bool(_) => todo!(),
                    TermLiteral::I8(_) => todo!(),
                    TermLiteral::I16(_) => todo!(),
                    TermLiteral::I32(i) => i.into(),
                    TermLiteral::I64(_) => todo!(),
                    TermLiteral::I128(_) => todo!(),
                    TermLiteral::ISize(_) => todo!(),
                    TermLiteral::U8(_) => todo!(),
                    TermLiteral::U16(_) => todo!(),
                    TermLiteral::U32(_) => todo!(),
                    TermLiteral::U64(_) => todo!(),
                    TermLiteral::U128(_) => todo!(),
                    TermLiteral::USize(lit) => (lit.value(self.db()) as usize).into(),
                    TermLiteral::R8(_) => todo!(),
                    TermLiteral::R16(_) => todo!(),
                    TermLiteral::R32(_) => todo!(),
                    TermLiteral::R64(_) => todo!(),
                    TermLiteral::R128(_) => todo!(),
                    TermLiteral::RSize(_) => todo!(),
                    TermLiteral::Nat(_) => todo!(),
                    TermLiteral::F32(lit) => lit.value(self.db()).into_inner().into(),
                    TermLiteral::F64(_) => todo!(),
                    TermLiteral::String(_) => todo!(),
                    TermLiteral::StaticLifetime => todo!(),
                };
                ValControlFlow::Continue(value)
            }
            ValOpn::ValItemLazilyDefined(_path) => {
                let expansion = val_repr.expansion(db).unwrap();
                self.eval_root_stmts(expansion.root_hir_lazy_stmt_val_reprs(db))
            }
            ValOpn::Linkage(linkage) => {
                let linkage_impl = self.comptime.linkage_impl(linkage);
                let control_flow = linkage_impl.eval(
                    val_repr.into(),
                    dev_eval_context::<Task::DevAscension>(),
                    unsafe { std::mem::transmute(val_repr.arguments(db) as &[ValArgumentRepr]) },
                );
                control_flow
            }
            ValOpn::FunctionGn(_) => todo!(),
            ValOpn::Prefix(_) => todo!(),
            ValOpn::Suffix(_) => todo!(),
            ValOpn::Binary(opr) => {
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 2);
                let ValArgumentRepr::Ordinary(lopd) = arguments[0] else {
                    unreachable!()
                };
                let ValArgumentRepr::Ordinary(ropd) = arguments[1] else {
                    unreachable!()
                };
                match opr {
                    HirBinaryOpr::Closed(_) => todo!(),
                    HirBinaryOpr::Shift(_) => todo!(),
                    HirBinaryOpr::Assign => todo!(),
                    HirBinaryOpr::AssignClosed(_) => todo!(),
                    HirBinaryOpr::AssignShift(_) => todo!(),
                    HirBinaryOpr::Comparison(opr) => {
                        let lopd = self.eval_val_repr(lopd)?;
                        let ropd = self.eval_val_repr(ropd)?;
                        ValControlFlow::Continue(
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
            ValOpn::EvalDiscarded => todo!(),
            ValOpn::Branches => {
                for val_argument_repr in val_repr.arguments(db) {
                    let ValArgumentRepr::Branch {
                        condition,
                        ref stmts,
                    } = *val_argument_repr
                    else {
                        unreachable!()
                    };
                    if let Some(condition) = condition {
                        if !self.eval_val_repr(condition)?.to_bool() {
                            continue;
                        }
                    }
                    return self.eval_stmts(stmts);
                }
                ValControlFlow::Continue(().into())
            }
            ValOpn::TypeVariant(path) => match path.index(db) {
                TypeVariantIndex::U8(index_raw) => {
                    ValControlFlow::Continue(TaskValue::<Task>::from_enum_u8(index_raw))
                }
            },
            ValOpn::Be { pattern_data } => {
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 1);
                let ValArgumentRepr::Ordinary(src) = arguments[0] else {
                    unreachable!()
                };
                let src = self.eval_val_repr(src)?;
                ValControlFlow::Continue(
                    match pattern_data {
                        ValPatternData::None => src.is_none(),
                        ValPatternData::Some => src.is_some(),
                    }
                    .into(),
                )
            }
            ValOpn::Unwrap {} => todo!(),
            ValOpn::Index => {
                // ad hoc
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 2);
                let ValArgumentRepr::Ordinary(owner) = arguments[0] else {
                    unreachable!()
                };
                let owner = self.eval_val_repr(owner)?;
                let ValArgumentRepr::Ordinary(index) = arguments[1] else {
                    unreachable!()
                };
                let index = self.eval_val_repr(index)?.to_usize();
                ValControlFlow::Continue(owner.index(index))
            }
        };
        result
    }

    fn eval_root_stmts(
        &self,
        stmt_val_reprs: &[ValRepr],
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        match self.eval_stmts(stmt_val_reprs) {
            ValControlFlow::Continue(value) | ValControlFlow::Return(value) => {
                ValControlFlow::Continue(value)
            }
            ValControlFlow::LoopContinue => unreachable!(),
            ValControlFlow::LoopExit(_) => unreachable!(),
            ValControlFlow::Undefined => unreachable!(),
            ValControlFlow::Err(e) => ValControlFlow::Err(e),
        }
    }

    fn eval_stmts(
        &self,
        stmt_val_reprs: &[ValRepr],
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        for &stmt_val_repr in &stmt_val_reprs[..stmt_val_reprs.len() - 1] {
            let _: () = self.eval_val_repr(stmt_val_repr)?.into();
        }
        self.eval_val_repr(*stmt_val_reprs.last().unwrap())
    }

    fn eval_val_argument(
        &self,
        val_argument_repr: &ValArgumentRepr,
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        match *val_argument_repr {
            ValArgumentRepr::Ordinary(val_repr) => self.eval_val_repr(val_repr),
            ValArgumentRepr::Keyed(_) => todo!(),
            ValArgumentRepr::Variadic(_) => todo!(),
            ValArgumentRepr::Branch {
                condition: _,
                stmts: _,
            } => todo!(),
            ValArgumentRepr::RuntimeConstants(_) => todo!(),
        }
    }
}

#[test]
fn val_repr_eval_works() {
    use husky_dev_comptime::DevComptime;
    use husky_ml_task::MlTask;
    use husky_ml_task_interface::InputId;
    use husky_path_utils::dev_paths::*;
    use husky_vfs::VfsDb;
    use husky_visual_protocol::trivial::TrivialVisualProtocol;

    let dev_paths = HuskyLangDevPaths::new();
    let runtime = DevRuntime::new(
        MlTask::<TrivialVisualProtocol>::new(),
        dev_paths.dev_root().join("examples/mnist-classifier"),
        None,
    )
    .unwrap();
    let db = runtime.db();
    let DevComptimeTarget::SingleCrate(crate_path) = runtime.comptime_target() else {
        unreachable!()
    };
    for &item_path in module_item_paths(db, crate_path.root_module_path(db)) {
        let ItemPath::MajorItem(MajorItemPath::Fugitive(fugitive_path)) = item_path else {
            continue;
        };
        let val_repr = ValRepr::new_val_item(fugitive_path, db);
        runtime.eval_val_repr_at_pedestal(val_repr, InputId::from_index(0).into());
    }
}
