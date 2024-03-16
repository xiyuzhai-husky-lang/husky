mod action;
mod once_use_rule;
mod use_all_rule;

pub use once_use_rule::*;
pub use use_all_rule::*;

pub(crate) use action::*;

use crate::*;
use husky_token::{TokenDb, TokenSheetData};
use vec_like::{AsVecMapEntry, VecPairMap};

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn item_tree_presheet(db: &::salsa::Db, module_path: ModulePath) -> EntityTreePresheet {
    EntityTreePresheetBuilder::new(db, module_path).build()
}

#[test]
fn item_tree_presheet_works() {
    DB::ast_expect_test_debug_with_db(
        |db, module_path| item_tree_presheet(db, module_path),
        &AstTestConfig::new(
            "item_tree_presheet",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    )
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreePresheet {
    module_path: ModulePath,
    major_item_node_table: MajorEntityNodeTable,
    once_use_rules: OnceUseRules,
    use_all_rules: UseAllRules,
    use_expr_arena: UseExprArena,
}

impl std::ops::Index<UseExprIdx> for EntityTreePresheet {
    type Output = UseExpr;

    fn index(&self, index: UseExprIdx) -> &Self::Output {
        &self.use_expr_arena[index]
    }
}

impl EntityTreePresheet {
    pub(crate) fn presheet_mut<'a>(&'a self, db: &'a ::salsa::Db) -> EntityTreePresheetMut<'a> {
        EntityTreePresheetMut {
            module_path: self.module_path,
            node_table: self.major_item_node_table.clone(),
            symbol_table: self.major_item_node_table.item_symbol_table(db),
            once_use_rules: self.once_use_rules.clone(),
            all_module_items_use_rules: self.use_all_rules.clone(),
            errors: vec![],
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
    once_use_rules: OnceUseRules,
    all_module_items_use_rules: UseAllRules,
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
        impl_block_syn_node_table: VecPairMap<ImplBlockSynNodePath, ImplBlockSynNode>,
    ) -> EntityTreeSheet {
        EntityTreeSheet::new(
            self.module_path,
            self.node_table,
            self.symbol_table,
            self.once_use_rules,
            self.all_module_items_use_rules,
            self.errors,
            impl_block_syn_node_table,
        )
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &::salsa::Db) {
        self.once_use_rules.check_done(db)
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
    item_use_trackers: OnceUseRules,
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

    fn build(mut self) -> EntityTreePresheet {
        for (ast_idx, ast) in self.ast_sheet.all_ast_indexed_iter() {
            self.process(ast_idx, ast)
        }
        EntityTreePresheet {
            module_path: self.module_path,
            major_item_node_table: self.item_node_table,
            once_use_rules: self.item_use_trackers,
            use_all_rules: Default::default(),
            use_expr_arena: self.use_expr_arena,
        }
    }

    fn process(&mut self, ast_idx: AstIdx, ast: &AstData) {
        match *ast {
            AstData::Use {
                token_verse_idx,
                ref visibility_expr,
                state_after_visibility_expr,
            } => {
                let mut token_stream = self
                    .token_sheet_data
                    .token_verse_token_stream(token_verse_idx, state_after_visibility_expr);
                let Ok(use_expr_root) =
                    parse_use_expr_root(&mut token_stream, &mut self.use_expr_arena)
                else {
                    return;
                };
                if let Some(new_rule) = OnceUseRule::new_root(
                    ast_idx,
                    use_expr_root,
                    visibility_expr,
                    &self.use_expr_arena,
                    self.module_path,
                ) {
                    self.item_use_trackers.push(new_rule)
                }
            }
            AstData::Identifiable {
                ref visibility_expr,
                ident_token,
                block,
                ..
            } => {
                let visibility = visibility_expr.visibility();
                if let Some(item_path) = block.item_path() {
                    self.item_node_table.try_add_new_node(
                        self.db,
                        &mut self.registry,
                        visibility,
                        ast_idx,
                        ident_token,
                        item_path,
                        block,
                    )
                }
            }
            AstData::Err { .. }
            | AstData::Sorc { .. }
            | AstData::Attr { .. }
            | AstData::BasicStmtOrBranch { .. }
            | AstData::IfElseStmts { .. }
            | AstData::MatchStmt { .. }
            | AstData::TypeVariant { .. }
            | AstData::ImplBlock { .. } => (),
        }
    }
}
