use super::*;

impl<'a, 'b> RustTranspilationBuilder<'a, 'b> {
    pub(crate) fn val_item_attr(&mut self, is_return_ty_always_copyable: bool) {
        use std::fmt::Write;
        let db = self.db;
        let task_dependency_ident = self
            .rust_transpilation_setup_data
            .task_dependency_ident
            .data(db);
        if is_return_ty_always_copyable {
            write!(self.result, "#[{}::val_item]\n", task_dependency_ident).unwrap()
        } else {
            write!(
                self.result,
                "#[{}::val_item_return_ref]\n",
                task_dependency_ident
            )
            .unwrap()
        }
    }

    pub(crate) fn memoized_field_attr(&mut self, is_return_ty_always_copyable: bool) {
        use std::fmt::Write;
        let db = self.db;
        let task_dependency_ident = self
            .rust_transpilation_setup_data
            .task_dependency_ident
            .data(db);
        if is_return_ty_always_copyable {
            write!(
                self.result,
                "#[{}::memoized_field]\n",
                task_dependency_ident
            )
            .unwrap()
        } else {
            write!(
                self.result,
                "#[{}::memoized_field_return_ref]\n",
                task_dependency_ident
            )
            .unwrap()
        }
    }
}
