use super::*;
use husky_entity_path::ItemPath;
use husky_entity_syn_tree::helpers::ingredient::HasIngredientIndex;

impl<'a, 'b> RustTranspilationBuilder<'a, 'b> {
    pub(crate) fn val_item_attr(
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
        let return_ref = !is_return_ty_always_copyable;
        write!(
            self.result,
            "#[{}::val_item(ingredient_index = {}{}{})]\n",
            task_dependency_ident,
            path.ingredient_index(db).unwrap().index(),
            is_lazy.then_some(", lazy").unwrap_or_default(),
            return_ref.then_some(", return_ref").unwrap_or_default(),
        )
        .unwrap()
    }

    pub(crate) fn memoized_field_attr(
        &mut self,
        path: ItemPath,
        is_return_ty_always_copyable: bool,
    ) {
        use std::fmt::Write;
        let db = self.db;
        let task_dependency_ident = self
            .rust_transpilation_setup_data
            .task_dependency_ident
            .data(db);
        if is_return_ty_always_copyable {
            write!(
                self.result,
                "#[{}::memoized_field({})]\n    ",
                task_dependency_ident,
                path.ingredient_index(db).unwrap().index()
            )
            .unwrap()
        } else {
            write!(
                self.result,
                "#[{}::memoized_field_return_ref({})]\n    ",
                task_dependency_ident,
                path.ingredient_index(db).unwrap().index()
            )
            .unwrap()
        }
    }
}
