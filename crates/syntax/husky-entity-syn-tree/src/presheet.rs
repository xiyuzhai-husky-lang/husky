mod action;
mod use_all_rule;
mod use_one_rule;

pub use use_all_rule::*;
pub use use_one_rule::*;

pub(crate) use action::*;

use crate::*;
use husky_token::{TokenDb, TokenSheetData};
use vec_like::{AsVecMapEntry, VecPairMap};

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn item_tree_presheet(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> EntitySynTreePresheet {
    EntityTreePresheetBuilder::new(db, module_path).build()
}

#[test]
fn item_tree_presheet_works() {
    DB::default().ast_expect_test_debug_with_db(
        |db, module_path| item_tree_presheet(db, module_path),
        &AstTestConfig::new("item_tree_presheet"),
    )
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct EntitySynTreePresheet {
    module_path: ModulePath,
    major_item_node_table: MajorEntityNodeTable,
    use_one_rules: UseOneRules,
    use_all_rules: UseAllRules,
    use_expr_arena: UseExprArena,
    errors: Vec<EntitySynTreeError>,
}

impl std::ops::Index<UseExprIdx> for EntitySynTreePresheet {
    type Output = UseExpr;

    fn index(&self, index: UseExprIdx) -> &Self::Output {
        &self.use_expr_arena[index]
    }
}

impl EntitySynTreePresheet {
    pub(crate) fn presheet_mut<'a>(&'a self, db: &'a ::salsa::Db) -> EntityTreePresheetMut<'a> {
        EntityTreePresheetMut {
            module_path: self.module_path,
            node_table: self.major_item_node_table.clone(),
            symbol_table: self.major_item_node_table.item_symbol_table(db),
            use_one_rules: self.use_one_rules.clone(),
            all_module_items_use_rules: self.use_all_rules.clone(),
            errors: self.errors.clone(),
            use_expr_arena: &self.use_expr_arena,
        }
    }

    pub(crate) fn major_item_node(&self, syn_node_path: ItemSynNodePath) -> Option<&ItemSynNode> {
        self.major_item_node_table.node(syn_node_path)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EntityTreePresheetMut<'a> {
    module_path: ModulePath,
    node_table: MajorEntityNodeTable,
    symbol_table: EntitySymbolTable,
    use_one_rules: UseOneRules,
    all_module_items_use_rules: UseAllRules,
    errors: Vec<EntitySynTreeError>,
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
        impl_block_syn_node_table: VecPairMap<ImplBlockSynNodePath, ImplBlockSynNode>,
    ) -> EntitySynTreeSheet {
        EntitySynTreeSheet::new(
            self.module_path,
            self.node_table,
            self.symbol_table,
            self.use_one_rules,
            self.all_module_items_use_rules,
            self.errors,
            impl_block_syn_node_table,
        )
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &::salsa::Db) {
        self.use_one_rules.check_done(db)
    }

    #[cfg(test)]
    pub fn use_all_rules(&self) -> &UseAllRules {
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
    db: &'a ::salsa::Db,
    module_path: ModulePath,
    ast_sheet: &'a AstSheet,
    token_sheet_data: &'a TokenSheetData,
    item_node_table: MajorEntityNodeTable,
    use_expr_arena: UseExprArena,
    item_use_trackers: UseOneRules,
    registry: ItemSynNodePathRegistry,
}

impl<'a> EntityTreePresheetBuilder<'a> {
    fn new(db: &'a ::salsa::Db, module_path: ModulePath) -> Self {
        Self {
            db,
            module_path,
            ast_sheet: module_path.ast_sheet(db),
            token_sheet_data: db.token_sheet_data(module_path),
            use_expr_arena: Default::default(),
            item_node_table: Default::default(),
            item_use_trackers: Default::default(),
            registry: Default::default(),
        }
    }

    fn build(mut self) -> EntitySynTreePresheet {
        for (ast_idx, ast) in self.ast_sheet.all_ast_indexed_iter() {
            self.process(ast_idx, ast)
        }
        EntitySynTreePresheet {
            module_path: self.module_path,
            major_item_node_table: self.item_node_table,
            use_one_rules: self.item_use_trackers,
            use_all_rules: Default::default(),
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
                    parse_use_expr_root(&mut token_stream, &mut self.use_expr_arena)
                else {
                    return;
                };
                if let Some(new_rule) = UseOneRule::new_root(
                    ast_idx,
                    use_expr_root,
                    visibility_expr,
                    &self.use_expr_arena,
                    self.module_path,
                ) {
                    self.item_use_trackers.push(new_rule)
                }
            }
            Ast::Identifiable {
                visibility_expr,
                ident_token,
                block,
                ..
            } => {
                let visibility = visibility_expr.visibility();
                let _ident = ident_token.ident();
                if let Some(item_path) = block.item_path() {
                    self.item_node_table.try_add_new_node(
                        self.db,
                        &mut self.registry,
                        visibility,
                        ast_idx,
                        *ident_token,
                        item_path,
                        *block,
                    )
                }
            }
            Ast::Err { .. }
            | Ast::Sorc { .. }
            | Ast::Attr { .. }
            | Ast::BasicStmtOrBranch { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmt { .. }
            | Ast::TypeVariant { .. }
            | Ast::ImplBlock { .. } => (),
        }
    }
}
