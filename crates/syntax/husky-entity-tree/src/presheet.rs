mod action;
mod use_all_rule;
mod use_expr_rule;

pub use use_all_rule::*;
pub use use_expr_rule::*;

use std::marker::PhantomData;

pub(crate) use action::*;
use husky_print_utils::p;
use husky_token::TokenSheetData;
use husky_word::IdentMap;

use crate::*;
use husky_doc::TextRange;
use vec_like::{AsVecMapEntry, InsertEntryRepeatError};

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_presheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> VfsResult<EntityTreePresheet> {
    Ok(EntityTreePresheetBuilder::new(db, module_path)?.build())
}

#[test]
fn entity_tree_presheet_works() {
    DB::default().vfs_expect_test_debug_with_db("entity_tree_presheet", |db, module_path| {
        entity_tree_presheet(db, module_path)
    })
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntityTreePresheet {
    module_path: ModulePath,
    native_symbol_entries: NativeEntitySymbolTable,
    use_one_trackers: UseExprRules,
    use_all_trackers: UseAllRules,
    use_expr_arena: UseExprArena,
    mod_path_arena: ModPathExprArena,
    errors: Vec<EntityTreeError>,
}

impl std::ops::Index<UseExprIdx> for EntityTreePresheet {
    type Output = UseExpr;

    fn index(&self, index: UseExprIdx) -> &Self::Output {
        &self.use_expr_arena[index]
    }
}

impl EntityTreePresheet {
    pub(crate) fn presheet_mut<'a>(
        &'a self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreePresheetMut<'a> {
        EntityTreePresheetMut {
            module_path: self.module_path,
            symbols: self.native_symbol_entries.entity_symbol_table(),
            use_expr_rules: self.use_one_trackers.clone(),
            use_all_rules: self.use_all_trackers.clone(),
            errors: self.errors.clone(),
            use_expr_arena: &self.use_expr_arena,
            mod_path_arena: &self.mod_path_arena,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EntityTreePresheetMut<'a> {
    module_path: ModulePath,
    symbols: EntitySymbolTable,
    use_expr_rules: UseExprRules,
    use_all_rules: UseAllRules,
    errors: Vec<EntityTreeError>,
    use_expr_arena: &'a UseExprArena,
    mod_path_arena: &'a ModPathExprArena,
}

impl<'a> EntityTreePresheetMut<'a> {
    pub(crate) fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub(crate) fn module_specific_symbols(&'a self) -> EntitySymbolTableRef<'a> {
        self.symbols.as_ref()
    }

    pub(crate) fn into_sheet(self, impls: Vec<Impl>) -> EntityTreeSheet {
        EntityTreeSheet::new(
            self.module_path,
            self.symbols,
            impls,
            self.use_expr_rules,
            self.use_all_rules,
            self.errors,
        )
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &dyn EntityTreeDb) {
        self.use_expr_rules.check_done(db)
    }
}

impl<'a> AsVecMapEntry for EntityTreePresheetMut<'a> {
    type K = ModulePath;
    fn key(&self) -> ModulePath {
        self.module_path
    }

    fn key_ref(&self) -> &ModulePath {
        &self.module_path
    }
}

struct EntityTreePresheetBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    ast_sheet: &'a AstSheet,
    module_path: ModulePath,
    native_symbol_entries: NativeEntitySymbolTable,
    mod_path_expr_arena: ModPathExprArena,
    use_expr_arena: UseExprArena,
    entity_use_trackers: UseExprRules,
    token_sheet_data: &'a TokenSheetData,
    errors: Vec<EntityTreeError>,
}

impl<'a> EntityTreePresheetBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            ast_sheet: db.ast_sheet(module_path)?,
            module_path,
            mod_path_expr_arena: Default::default(),
            use_expr_arena: Default::default(),
            native_symbol_entries: Default::default(),
            entity_use_trackers: Default::default(),
            token_sheet_data: db.token_sheet_data(module_path).unwrap(),
            errors: vec![],
        })
    }

    fn build(mut self) -> EntityTreePresheet {
        for (ast_idx, ast) in self.ast_sheet.all_ast_indexed_iter() {
            self.process(ast_idx, ast)
        }
        EntityTreePresheet {
            module_path: self.module_path,
            native_symbol_entries: self.native_symbol_entries,
            use_one_trackers: self.entity_use_trackers,
            use_all_trackers: Default::default(),
            errors: self.errors,
            use_expr_arena: self.use_expr_arena,
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
                let Ok(use_expr_root) =
                    parse_use_expr_root(&mut token_stream, &mut self.use_expr_arena) else {
                        todo!()
                    };
                let use_expr_idx = use_expr_root.use_expr_idx();
                if let Some(new_rule) = UseExprRule::new_root(
                    ast_idx,
                    use_expr_idx,
                    accessibility_expr,
                    &self.use_expr_arena[use_expr_idx],
                    self.module_path,
                ) {
                    self.entity_use_trackers.push(new_rule)
                }
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
                    match self.native_symbol_entries.insert(new_entry) {
                        Ok(_) => (),
                        Err(e) => {
                            todo!()
                            // let old_native_symbol_entry = &self.native_symbol_entries.data()[e.old];
                            // self.errors
                            //     .push(EntityTreeError::EntitySymbolAlreadyDefined {
                            //         old: old_native_symbol_entry.symbol().ast_idx(self.db),
                            //         new: ast_idx,
                            //     })
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
