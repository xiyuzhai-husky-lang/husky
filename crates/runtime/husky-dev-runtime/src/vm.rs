use husky_entity_path::path::major_item::form::MajorFormPath;
use husky_linket_impl::LinketImplVmControlFlowThawed;

use crate::*;

impl<Devsoul: IsDevsoul> IsVmRuntime<Devsoul::LinketImpl> for DevRuntime<Devsoul> {
    type Linktime = Devsoul::Linktime;

    fn linktime(&self) -> &Self::Linktime {
        self.comptime().linktime()
    }

    fn eval_val(&self, path: MajorFormPath) -> LinketImplVmControlFlowThawed<Devsoul::LinketImpl> {
        let db = self.db();
        let ki_repr = KiRepr::new_val(path, db);
        self.eval_ki_repr(ki_repr).into_vm().unwrap()
    }
}
