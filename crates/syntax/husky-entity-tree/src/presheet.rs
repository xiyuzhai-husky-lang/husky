mod action;
mod entry;
mod use_all_rule;
mod use_expr_rule;

pub use entry::*;

use std::marker::PhantomData;

pub(crate) use action::*;
use husky_print_utils::p;
use husky_token::TokenSheetData;
use husky_word::IdentMap;

use crate::*;
use husky_text::TextRange;
use use_all_rule::*;
use use_expr_rule::*;
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
    native_symbol_entries: IdentMap<NativeEntitySymbolEntry>,
    use_one_trackers: UseTreeRules,
    use_all_trackers: UseAllRules,
    use_tree_expr_arena: UseExprArena,
    mod_path_arena: ModPathExprArena,
    errors: Vec<EntityTreeError>,
}

impl EntreePresheet {
    pub(crate) fn presheet_mut<'a>(&'a self, db: &'a dyn EntityTreeDb) -> EntreePresheetMut<'a> {
        EntreePresheetMut {
            module_path: self.module_path,
            symbols: self
                .native_symbol_entries
                .iter()
                .map(|entry| entry.into())
                .collect(),
            use_expr_rules: self.use_one_trackers.clone(),
            use_all_rules: self.use_all_trackers.clone(),
            errors: self.errors.clone(),
            use_tree_expr_arena: &self.use_tree_expr_arena,
            mod_path_arena: &self.mod_path_arena,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EntreePresheetMut<'a> {
    module_path: ModulePath,
    symbols: IdentMap<EntitySymbolEntry>,
    use_expr_rules: UseTreeRules,
    use_all_rules: UseAllRules,
    errors: Vec<EntityTreeError>,
    use_tree_expr_arena: &'a UseExprArena,
    mod_path_arena: &'a ModPathExprArena,
}

impl<'a> EntreePresheetMut<'a> {
    pub(crate) fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub(crate) fn module_specific_symbols(&self) -> &IdentMap<EntitySymbolEntry> {
        &self.symbols
    }

    pub(crate) fn into_sheet(self, impl_blocks: Vec<ImplBlock>) -> EntityTreeModuleSheet {
        EntityTreeModuleSheet::new(self.module_path, self.symbols, impl_blocks)
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &dyn EntityTreeDb) {
        self.use_expr_rules.check_done(db)
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
    native_symbol_entries: VecMap<NativeEntitySymbolEntry>,
    mod_path_expr_arena: ModPathExprArena,
    use_tree_expr_arena: UseExprArena,
    entity_use_trackers: UseTreeRules,
    token_sheet_data: &'a TokenSheetData,
    errors: Vec<EntityTreeError>,
}

impl<'a> EntreePresheetBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            ast_sheet: db.ast_sheet(module_path)?,
            module_path,
            mod_path_expr_arena: Default::default(),
            use_tree_expr_arena: Default::default(),
            native_symbol_entries: Default::default(),
            entity_use_trackers: Default::default(),
            token_sheet_data: db.token_sheet_data(module_path).unwrap(),
            errors: vec![],
        })
    }

    fn build(mut self) -> EntreePresheet {
        for (ast_idx, ast) in self.ast_sheet.all_ast_indexed_iter() {
            self.process(ast_idx, ast)
        }
        EntreePresheet {
            module_path: self.module_path,
            native_symbol_entries: self.native_symbol_entries,
            use_one_trackers: self.entity_use_trackers,
            use_all_trackers: Default::default(),
            errors: self.errors,
            use_tree_expr_arena: self.use_tree_expr_arena,
            mod_path_arena: self.mod_path_expr_arena,
        }
    }

    fn process(&mut self, ast_idx: AstIdx, ast: &Ast) {
        match ast {
            Ast::Use { token_group_idx } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(*token_group_idx, None);
                // let module_symbol_context = self.module_symbol_context;
                let accessibility_expr = match parse_accessibility_expr(
                    &mut token_stream,
                    &mut self.mod_path_expr_arena,
                ) {
                    Ok(accessibility_expr) => accessibility_expr,
                    Err(_) => todo!(),
                };
                let Ok(use_tree_expr_root) =
                    parse_use_tree_expr_root(&mut token_stream, &mut self.use_tree_expr_arena) else {
                        todo!()
                    };
                self.entity_use_trackers.push(UseTreeRule::new_root(
                    ast_idx,
                    accessibility_expr,
                    &self.use_tree_expr_arena[use_tree_expr_root.use_tree_expr_idx()],
                    self.module_path,
                ))
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
                let accessibility = *accessibility;
                let ident = ident_token.ident();
                if let Some(entity_path) = entity_path {
                    let new_entry = NativeEntitySymbolEntry::new(
                        ident,
                        accessibility,
                        match entity_path {
                            EntityPath::Module(module_path) => {
                                SubmoduleSymbol::new(self.db, *module_path, accessibility, ast_idx)
                                    .into()
                            }
                            EntityPath::ModuleItem(module_item_path) => ModuleItemSymbol::new(
                                self.db,
                                *module_item_path,
                                accessibility,
                                ast_idx,
                            )
                            .into(),
                            EntityPath::AssociatedItem(_) | EntityPath::Variant(_) => return,
                        },
                    );
                    match self.native_symbol_entries.insert_new(new_entry) {
                        Ok(_) => (),
                        Err(e) => {
                            let old_native_symbol_entry = &self.native_symbol_entries.data()[e.old];
                            self.errors
                                .push(EntityTreeError::EntitySymbolAlreadyDefined {
                                    old: old_native_symbol_entry.symbol().ast_idx(self.db),
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
                &(&self.native_symbol_entries).debug_with(db, include_all_fields),
            )
            .field(
                "entity_use_roots",
                &self.use_one_trackers.debug_with(db, include_all_fields),
            )
            .finish()
    }
}
