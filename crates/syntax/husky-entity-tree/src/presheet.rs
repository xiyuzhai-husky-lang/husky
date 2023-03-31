mod action;
mod use_all_rule;
mod use_expr_rule;

pub use use_all_rule::*;
pub use use_expr_rule::*;

pub(crate) use action::*;

use husky_token::TokenSheetData;

use crate::*;

use vec_like::AsVecMapEntry;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_presheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> VfsResult<EntityTreePresheet> {
    Ok(EntityTreePresheetBuilder::new(db, module_path)?.build())
}

#[test]
fn entity_tree_presheet_works() {
    DB::default().ast_expect_test_debug_with_db("entity_tree_presheet", |db, module_path| {
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
    mod_path_arena: ModulePathExprArena,
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
        _db: &'a dyn EntityTreeDb,
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
    mod_path_arena: &'a ModulePathExprArena,
}

impl<'a> EntityTreePresheetMut<'a> {
    pub(crate) fn module_path(&self) -> ModulePath {
        self.module_path
    }

    /// symbols in module except those from prelude
    pub(crate) fn module_specific_symbols(&'a self) -> EntitySymbolTableRef<'a> {
        self.symbols.as_ref()
    }

    pub(crate) fn into_sheet(self, impl_blocks: Vec<ImplBlock>) -> EntityTreeSheet {
        EntityTreeSheet::new(
            self.module_path,
            self.symbols,
            self.use_expr_rules,
            self.use_all_rules,
            self.errors,
            impl_blocks,
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
    mod_path_expr_arena: ModulePathExprArena,
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
                        return
                    };
                if let Some(new_rule) = UseExprRule::new_root(
                    ast_idx,
                    use_expr_root,
                    accessibility_expr,
                    &self.use_expr_arena,
                    self.module_path,
                ) {
                    self.entity_use_trackers.push(new_rule)
                }
            }
            Ast::Defn {
                accessibility,
                entity_path,
                ident_token,
                ..
            } => {
                let accessibility = *accessibility;
                let ident = ident_token.ident();
                if let Some(entity_path) = entity_path {
                    let new_entry = NativeEntitySymbolEntry::new(
                        ident,
                        accessibility,
                        match entity_path {
                            EntityPath::Module(module_path) => SubmoduleSymbol::new(
                                self.db,
                                *module_path,
                                accessibility,
                                ast_idx,
                                *ident_token,
                            )
                            .into(),
                            EntityPath::ModuleItem(module_item_path) => ModuleItemSymbol::new(
                                self.db,
                                *module_item_path,
                                accessibility,
                                ast_idx,
                                *ident_token,
                            )
                            .into(),
                            EntityPath::AssociatedItem(_) | EntityPath::Variant(_) => return,
                        },
                    );
                    self.native_symbol_entries
                        .insert(self.db, new_entry)
                        .map_err(|e| self.errors.push(e));
                }
            }
            Ast::Err { .. }
            | Ast::Attr { .. }
            | Ast::Decr { .. }
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
