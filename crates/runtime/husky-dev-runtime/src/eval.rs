use crate::*;
use husky_entity_path::{ItemPath, MajorItemPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_fluffy_term::FluffyTermEngine;
use husky_task::{helpers::TaskValue, IsTask};
use husky_val::ValOpn;
use husky_val_repr::repr::ValRepr;
use husky_vfs::PackagePath;

impl<Task: IsTask> DevRuntime<Task> {
    pub fn eval_val_at_base_point(
        &self,
        val_repr: ValRepr,
        base_point: TaskDevAscensionBasePoint<Task>,
    ) -> (ValControlFlow, TaskValue<Task>) {
        with_eval_context::<TaskDevAscension<Task>, _, _>(self, base_point, || {
            self.eval_val(val_repr)
        })
    }

    fn eval_val(&self, val_repr: ValRepr) -> (ValControlFlow, TaskValue<Task>) {
        let db = self.db();
        match val_repr.opn(db) {
            ValOpn::Return => todo!(),
            ValOpn::Require => todo!(),
            ValOpn::Assert => todo!(),
            ValOpn::Literal(_) => todo!(),
            ValOpn::ValItemLazilyDefined(_) => todo!(),
            ValOpn::Linkage(_) => todo!(),
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
}

pub enum ValControlFlow {
    Continue,
    LoopContinue,
    LoopBreak,
    Return,
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
