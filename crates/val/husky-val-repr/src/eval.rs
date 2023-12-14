use crate::*;
use husky_task::{helpers::TaskValue, IsTask};
use husky_val::ValOpn;
use husky_vfs::PackagePath;

impl ValRepr {
    pub fn eval<Task: IsTask>(self, db: &::salsa::Db) -> (ValControlFlow, TaskValue<Task>) {
        match self.opn(db) {
            ValOpn::Return => todo!(),
            ValOpn::Require => todo!(),
            ValOpn::Assert => todo!(),
            ValOpn::Literal(_) => todo!(),
            ValOpn::ValItem(_) => todo!(),
            ValOpn::LinkageImpl(_) => todo!(),
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
    use husky_vfs::VfsDb;
    let mut db = DB::default();
    db.ast_plain_test(
        |db, package_path: PackagePath| match package_path.name_str(db) {
            "mnist-classifier" => {
                package_path
                    .main_crate_path(db)
                    .unwrap()
                    .root_module_path(db);
                // todo!();
                // todo!()
            }
            _ => (),
        },
        &AstTestConfig::new("val_repr_eval"),
    )
}
