mod action;
mod module_use_all_rule;
mod once_use_rule;

pub use module_use_all_rule::*;
pub use once_use_rule::*;

pub(crate) use action::*;

use crate::*;
use husky_token::TokenSheetData;
use vec_like::{AsVecMapEntry, VecPairMap};

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
    major_entity_node_table: MajorEntityNodeTable,
    use_one_trackers: OnceUseRules,
    use_all_trackers: UseAllModuleSymbolsRules,
    use_expr_arena: UseExprArena,
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
            node_table: self.major_entity_node_table.clone(),
            symbol_table: self.major_entity_node_table.entity_symbol_table(db),
            once_use_rules: self.use_one_trackers.clone(),
            all_module_items_use_rules: self.use_all_trackers.clone(),
            errors: self.errors.clone(),
            use_expr_arena: &self.use_expr_arena,
        }
    }

    pub(crate) fn major_entity_node(&self, node_path: EntityNodePath) -> Option<EntityNode> {
        self.major_entity_node_table.node(node_path)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EntityTreePresheetMut<'a> {
    module_path: ModulePath,
    node_table: MajorEntityNodeTable,
    symbol_table: EntitySymbolTable,
    once_use_rules: OnceUseRules,
    all_module_items_use_rules: UseAllModuleSymbolsRules,
    errors: Vec<EntityTreeError>,
    use_expr_arena: &'a UseExprArena,
}

impl<'a> EntityTreePresheetMut<'a> {
    pub(crate) fn module_path(&self) -> ModulePath {
        self.module_path
    }

    /// symbols in module except those from prelude
    pub(crate) fn module_specific_symbols(&'a self) -> EntitySymbolTableRef<'a> {
        self.symbol_table.as_ref()
    }

    pub(crate) fn into_sheet(
        self,
        impl_block_node_table: VecPairMap<ImplBlockNodePath, ImplBlockNode>,
    ) -> EntityTreeSheet {
        EntityTreeSheet::new(
            self.module_path,
            self.node_table,
            self.symbol_table,
            self.once_use_rules,
            self.all_module_items_use_rules,
            self.errors,
            impl_block_node_table,
        )
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &dyn EntityTreeDb) {
        self.once_use_rules.check_done(db)
    }

    #[cfg(test)]
    pub(crate) fn use_all_rules(&self) -> &UseAllModuleSymbolsRules {
        &self.all_module_items_use_rules
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
    module_path: ModulePath,
    ast_sheet: &'a AstSheet,
    token_sheet_data: &'a TokenSheetData,
    entity_node_table: MajorEntityNodeTable,
    use_expr_arena: UseExprArena,
    entity_use_trackers: OnceUseRules,
    registry: EntityNodeRegistry,
}

impl<'a> EntityTreePresheetBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            module_path,
            ast_sheet: db.ast_sheet(module_path)?,
            token_sheet_data: db.token_sheet_data(module_path).unwrap(),
            use_expr_arena: Default::default(),
            entity_node_table: Default::default(),
            entity_use_trackers: Default::default(),
            registry: Default::default(),
        })
    }

    fn build(mut self) -> EntityTreePresheet {
        for (ast_idx, ast) in self.ast_sheet.all_ast_indexed_iter() {
            self.process(ast_idx, ast)
        }
        EntityTreePresheet {
            module_path: self.module_path,
            major_entity_node_table: self.entity_node_table,
            use_one_trackers: self.entity_use_trackers,
            use_all_trackers: Default::default(),
            use_expr_arena: self.use_expr_arena,
            errors: self.registry.finish_with_errors(),
        }
    }

    fn process(&mut self, ast_idx: AstIdx, ast: &Ast) {
        match ast {
            Ast::Use {
                token_group_idx,
                visibility_expr,
                state_after_visibility_expr,
            } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(*token_group_idx, *state_after_visibility_expr);
                let Ok(use_expr_root) =
                    parse_use_expr_root(&mut token_stream, &mut self.use_expr_arena) else {
                        return
                    };
                if let Some(new_rule) = OnceUseRule::new_root(
                    ast_idx,
                    use_expr_root,
                    visibility_expr,
                    &self.use_expr_arena,
                    self.module_path,
                ) {
                    self.entity_use_trackers.push(new_rule)
                }
            }
            Ast::Defn {
                visibility_expr,
                ident_token,
                block,
                ..
            } => {
                let visibility = visibility_expr.visibility();
                let ident = ident_token.ident();
                if let Some(entity_path) = block.entity_path() {
                    self.entity_node_table.try_add_new_node(
                        self.db,
                        &mut self.registry,
                        visibility,
                        ast_idx,
                        *ident_token,
                        entity_path,
                    )
                }
            }
            Ast::Err { .. }
            | Ast::Attr { .. }
            | Ast::Decr { .. }
            | Ast::BasicStmtOrBranch { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. }
            | Ast::TypeVariant { .. }
            | Ast::ImplBlock { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => (),
        }
    }
}
