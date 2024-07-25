use super::*;
use husky_entity_path::path::ItemPath;
use husky_fmt_utils::FmtPunctuated;
use husky_sem_var_deps::{item_sem_var_deps, var_deps::SemVarDep};
use mangle::item_path_id_interface_cache_path;
use salsa::DisplayWithDb;

impl<'a, 'b> RustTranspilationBuilder<'a, 'b> {
    pub(crate) fn val_attr(
        &mut self,
        path: ItemPath,
        is_lazy: bool,
        is_return_ty_always_copyable: bool,
    ) {
        use std::fmt::Write;
        let db = self.db;
        let task_dependency_ident = self
            .rust_transpilation_setup_data
            .task_dependency_ident
            .data(db);
        let var_deps = item_sem_var_deps(path, db);
        let return_leash = !is_return_ty_always_copyable;
        write!(
            self.result,
            r#"#[{}::val(
    item_path_id_interface = {},
    var_deps = [{}]{}{}
)]
"#,
            task_dependency_ident,
            item_path_id_interface_cache_path(path, db).unwrap(),
            FmtPunctuated::new(&var_deps, ", ", |&dep, f| match dep {
                SemVarDep::Item(path) => path.display_fmt_with_db(f, db),
            }),
            is_lazy.then_some(",\n    lazy").unwrap_or_default(),
            return_leash
                .then_some(",\n    return_leash")
                .unwrap_or_default(),
        )
        .unwrap()
    }

    pub(crate) fn memo_field_attr(&mut self, path: ItemPath, is_return_ty_always_copyable: bool) {
        use std::fmt::Write;
        let db = self.db;
        let task_dependency_ident = self
            .rust_transpilation_setup_data
            .task_dependency_ident
            .data(db);
        let return_leash = !is_return_ty_always_copyable;
        write!(
            self.result,
            "#[{}::memo(item_path_id_interface = {}{})]\n    ",
            task_dependency_ident,
            item_path_id_interface_cache_path(path, db).unwrap(),
            return_leash.then_some(", return_leash").unwrap_or_default(),
        )
        .unwrap()
    }
}
