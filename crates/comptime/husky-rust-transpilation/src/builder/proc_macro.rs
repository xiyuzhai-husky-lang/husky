use super::*;
use husky_entity_path::path::ItemPath;
use husky_entity_tree::helpers::ingredient::HasIngredientIndex;

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
        let return_leash = !is_return_ty_always_copyable;
        write!(
            self.result,
            "#[{}::val(ingredient_index = {}{}{})]\n",
            task_dependency_ident,
            path.ingredient_index(db).unwrap().index(),
            is_lazy.then_some(", lazy").unwrap_or_default(),
            return_leash.then_some(", return_leash").unwrap_or_default(),
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
            "#[{}::memo(ingredient_index = {}{})]\n    ",
            task_dependency_ident,
            path.ingredient_index(db).unwrap().index(),
            return_leash.then_some(", return_leash").unwrap_or_default(),
        )
        .unwrap()
    }
}
