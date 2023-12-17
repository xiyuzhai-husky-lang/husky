use crate::*;
use husky_entity_path::{ItemPath, MajorItemPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_fluffy_term::FluffyTermEngine;
use husky_task::{
    dev_ascension::{dev_eval_context, with_runtime_and_base_point},
    helpers::TaskValue,
    IsTask,
};
use husky_task_prelude::IsLinkageImpl;
use husky_val::ValOpn;
use husky_val_repr::repr::{ValArgumentRepr, ValRepr};
use husky_vfs::PackagePath;
use smallvec::*;
use std::{
    convert::Infallible,
    ops::{FromResidual, Try},
};

impl<Task: IsTask> DevRuntime<Task> {
    pub fn eval_val_at_base_point(
        &self,
        val_repr: ValRepr,
        base_point: TaskDevBasePoint<Task>,
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>> {
        with_runtime_and_base_point::<TaskDevAscension<Task>, _, _>(self, base_point, || {
            self.eval_val(val_repr)
        })
    }

    fn eval_val(&self, val_repr: ValRepr) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>> {
        let db = self.db();
        let arguments: SmallVec<[TaskValue<Task>; 4]> = val_repr
            .arguments(db)
            .iter()
            .map(|val_argument_repr| self.eval_val_argument(val_argument_repr))
            .collect::<ValControlFlow<TaskValue<Task>, SmallVec<[TaskValue<Task>; 4]>>>()?;
        match val_repr.opn(db) {
            ValOpn::Return => todo!(),
            ValOpn::Require => todo!(),
            ValOpn::Assert => todo!(),
            ValOpn::Literal(_) => todo!(),
            ValOpn::ValItemLazilyDefined(_) => todo!(),
            ValOpn::Linkage(linkage) => {
                let linkage_impl = self.comptime.linkage_impl(linkage);
                linkage_impl.eval_fn(dev_eval_context::<Task::DevAscension>(), Default::default());
                todo!()
            }
            ValOpn::FunctionGn(_) => todo!(),
            ValOpn::Prefix(_) => todo!(),
            ValOpn::Suffix(_) => todo!(),
            ValOpn::Binary(_) => todo!(),
            ValOpn::EvalDiscarded => todo!(),
            ValOpn::NewList => todo!(),
            ValOpn::Branches => todo!(),
            ValOpn::TypeVariant(_) => todo!(),
            ValOpn::Be => todo!(),
        }
    }

    fn eval_val_argument(
        &self,
        val_argument_repr: &ValArgumentRepr,
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>> {
        match val_argument_repr {
            ValArgumentRepr::Ordinary(_) => todo!(),
            ValArgumentRepr::Keyed(_, _) => todo!(),
            ValArgumentRepr::Variadic(_) => todo!(),
            ValArgumentRepr::Branch { condition, stmts } => todo!(),
        }
    }
}

pub enum ValControlFlow<B, C> {
    Continue(C),
    LoopContinue,
    LoopBreak(B),
    Return(B),
}

impl<B, C> std::ops::Residual<C> for ValControlFlow<B, Infallible> {
    type TryType = ValControlFlow<B, C>;
}

impl<C, B> std::ops::Try for ValControlFlow<B, C> {
    type Output = C;

    type Residual = ValControlFlow<B, Infallible>;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            ValControlFlow::Continue(c) => std::ops::ControlFlow::Continue(c),
            ValControlFlow::LoopContinue => {
                std::ops::ControlFlow::Break(ValControlFlow::LoopContinue)
            }
            ValControlFlow::LoopBreak(b) => {
                std::ops::ControlFlow::Break(ValControlFlow::LoopBreak(b))
            }
            ValControlFlow::Return(b) => std::ops::ControlFlow::Break(ValControlFlow::LoopBreak(b)),
        }
    }
}

impl<B, C> std::ops::FromResidual<ValControlFlow<B, Infallible>> for ValControlFlow<B, C> {
    fn from_residual(residual: ValControlFlow<B, Infallible>) -> Self {
        todo!()
    }
}

impl<B, C1, C2: FromIterator<C1>> std::iter::FromIterator<ValControlFlow<B, C1>>
    for ValControlFlow<B, C2>
{
    fn from_iter<T: IntoIterator<Item = ValControlFlow<B, C1>>>(iter: T) -> Self {
        match iter
            .into_iter()
            .map(|item| match item.branch() {
                std::ops::ControlFlow::Continue(c1) => Ok(c1),
                std::ops::ControlFlow::Break(residual) => Err(residual),
            })
            .collect::<Result<C2, ValControlFlow<B, Infallible>>>()
        {
            Ok(c2) => ValControlFlow::Continue(c2),
            Err(residual) => ValControlFlow::from_residual(residual),
        }
    }
}

#[test]
fn val_repr_eval_works() {
    use husky_dev_comptime::DevComptime;
    use husky_ml_task::MlTask;
    use husky_ml_task_prelude::SampleId;
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
    let DevComptimeTarget::SingleCrate(crate_path) = runtime.target() else {
        unreachable!()
    };
    for &item_path in module_item_paths(db, crate_path.root_module_path(db)) {
        let ItemPath::MajorItem(MajorItemPath::Fugitive(fugitive_path)) = item_path else {
            continue;
        };
        let val_repr = ValRepr::new_val_item(fugitive_path, db);
        runtime.eval_val_at_base_point(val_repr, SampleId::from_index(0));
    }
}
