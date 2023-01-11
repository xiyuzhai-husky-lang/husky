mod action;
mod use_all_tracker;
mod use_expr_tracker;

use std::marker::PhantomData;

pub(crate) use action::*;
use husky_print_utils::p;

use crate::{ModuleItem, *};
use husky_text::TextRange;
use use_all_tracker::*;
use use_expr_tracker::*;
use vec_like::{AsVecMapEntry, InsertEntryRepeatError};

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entree_presheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> VfsResult<EntreePresheet> {
    Ok(EntreePresheetBuilder::new(db, module_path)?.build())
}

#[test]
fn entree_presheet_works() {
    DB::expect_test_probable_modules_debug_ref_with_db("entity_tree_presheet", |db, module_path| {
        entree_presheet(db, module_path)
    })
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EntreePresheet {
    module_path: ModulePath,
    module_specific_symbols: VecMap<EntitySymbol>,
    use_one_trackers: UseExprTrackers,
    use_all_trackers: UseAllTrackers,
    errors: Vec<EntityTreeError>,
}

impl EntreePresheet {
    pub(crate) fn presheet_mut<'a>(&self, db: &'a dyn EntityTreeDb) -> EntreePresheetMut<'a> {
        EntreePresheetMut {
            module_path: self.module_path,
            module_specific_symbols: self.module_specific_symbols.clone(),
            use_expr_trackers: self.use_one_trackers.clone(),
            use_all_trackers: self.use_all_trackers.clone(),
            errors: self.errors.clone(),
            use_tree_expr_arena: Default::default(),
            mod_path_arena: Default::default(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EntreePresheetMut<'a> {
    module_path: ModulePath,
    module_specific_symbols: VecMap<EntitySymbol>,
    use_expr_trackers: UseExprTrackers,
    use_all_trackers: UseAllTrackers,
    errors: Vec<EntityTreeError>,
    use_tree_expr_arena: UseTreeExprArena,
    mod_path_arena: ModPathExprArena,
    // placeholder; to be removed later
    phantom: PhantomData<&'a ()>,
}

impl<'a> EntreePresheetMut<'a> {
    pub(crate) fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub(crate) fn module_specific_symbols(&self) -> &VecMap<EntitySymbol> {
        &self.module_specific_symbols
    }

    pub(crate) fn into_sheet(self, impl_blocks: Vec<ImplBlock>) -> EntityTreeModuleSheet {
        EntityTreeModuleSheet::new(self.module_path, self.module_specific_symbols, impl_blocks)
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &dyn EntityTreeDb) {
        self.use_expr_trackers.check_done(db)
    }
}

impl<'a> AsVecMapEntry for EntreePresheetMut<'a> {
    type K = ModulePath;
    fn key(&self) -> ModulePath {
        self.module_path
    }

    fn key_ref(&self) -> &ModulePath {
        &self.module_path
    }
}

struct EntreePresheetBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    ast_sheet: &'a AstSheet,
    module_path: ModulePath,
    nodes: VecMap<EntitySymbol>,
    entity_use_trackers: UseExprTrackers,
    errors: Vec<EntityTreeError>,
}

impl<'a> EntreePresheetBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            ast_sheet: db.ast_sheet(module_path)?,
            module_path,
            nodes: Default::default(),
            entity_use_trackers: Default::default(),
            errors: vec![],
        })
    }

    fn build(mut self) -> EntreePresheet {
        for (ast_idx, ast) in self.ast_sheet.all_ast_indexed_iter() {
            self.process(ast_idx, ast)
        }
        EntreePresheet {
            module_path: self.module_path,
            module_specific_symbols: self.nodes,
            use_one_trackers: self.entity_use_trackers,
            use_all_trackers: Default::default(),
            errors: self.errors,
        }
    }

    fn process(&mut self, ast_idx: AstIdx, ast: &Ast) {
        match ast {
            Ast::Use { token_group_idx } => {
                // let module_symbol_context = self.module_symbol_context;
                todo!();
                // self.entity_use_trackers.push(UseExprTracker::new_root(
                //     ast_idx,
                //     *accessibility,
                //     *ident,
                //     *use_expr_idx,
                // ))
            }
            Ast::Defn {
                token_group_idx,
                body,
                accessibility,
                entity_kind,
                entity_path,
                is_generic,
                body_kind,
                saved_stream_state,
                ident_token,
            } => {
                let ident = ident_token.ident();
                if let Some(entity_path) = entity_path {
                    let new_node = match entity_path {
                        EntityPath::Module(module_path) => EntitySymbol::Submodule {
                            ident,
                            accessibility: *accessibility,
                            module_path: *module_path,
                            ast_idx,
                        },
                        EntityPath::ModuleItem(module_item_path) => EntitySymbol::ModuleItem(
                            ModuleItem::new(ident, *accessibility, *module_item_path, ast_idx),
                        ),
                        EntityPath::AssociatedItem(_) | EntityPath::Variant(_) => return,
                    };
                    match self.nodes.insert_new(new_node) {
                        Ok(_) => (),
                        Err(e) => {
                            let old_node = &self.nodes.data()[e.old];
                            self.errors
                                .push(EntityTreeError::EntitySymbolAlreadyDefined {
                                    old: old_node.ast_idx().unwrap(),
                                    new: ast_idx,
                                })
                        }
                    }
                }
            }
            Ast::Err { .. }
            | Ast::Decor { .. }
            | Ast::BasicStmtOrBranch { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. }
            | Ast::ModuleItemVariant { .. }
            | Ast::Impl { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => (),
        }
    }
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntreePresheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        f.debug_struct("EntityTreePresheet")
            .field(
                "module_path",
                &self
                    .module_path
                    .debug_with(db as &dyn VfsDb, include_all_fields),
            )
            .field(
                "module_specific_symbols",
                &(&self.module_specific_symbols).debug_with(db, include_all_fields),
            )
            .field(
                "entity_use_roots",
                &self.use_one_trackers.debug_with(db, include_all_fields),
            )
            .finish()
    }
}
