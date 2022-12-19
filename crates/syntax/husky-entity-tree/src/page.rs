/// primal doesn't care about uses and impls
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreePage {
    arena: EntityTreeArena,
    top_level_entities_idx_range: EntityTreeIdxRange,
    isolate_entities_idx_range: EntityTreeIdxRange,
}

impl EntityTreePage {
    pub fn show(&self, db: &dyn EntityTreeDb) -> String {
        let all_entity_paths: Vec<_> = self
            .arena
            .data()
            .iter()
            .map(|tree| tree.node.entity_path().show(db))
            .collect();
        format!("{:?}", all_entity_paths)
    }
}

impl EntityTreePage {
    pub(crate) fn get(&self, entity_path: EntityPath) -> Option<&EntityTree> {
        self.arena
            .data()
            .iter()
            .find(|node| node.entity_path() == entity_path)
    }

    pub(crate) fn top_level_entities<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Accessibility, EntityCard, EntityPath)> + 'a {
        self[&self.top_level_entities_idx_range].iter().map(|tree| {
            (
                tree.node.accessibility(),
                tree.node.card(),
                tree.node.entity_path(),
            )
        })
    }
}

impl std::ops::Deref for EntityTreePage {
    type Target = EntityTreeArena;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_page(
    db: &dyn EntityTreeDb,
    module: EntityPath,
) -> VfsResult<EntityTreePage> {
    let ast_sheet = db.ast_sheet(module).as_ref()?;
    EntityTreePageBuilder::new(db, module, ast_sheet).build()
}

struct EntityTreePageBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    module: EntityPath,
    ast_sheet: &'a AstSheet,
    arena: EntityTreeArena,
    sporadic_entities: Vec<EntityTree>,
}

impl<'a> EntityTreePageBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, module: EntityPath, ast_sheet: &'a AstSheet) -> Self {
        Self {
            db,
            module,
            ast_sheet,
            arena: Default::default(),
            sporadic_entities: Default::default(),
        }
    }

    fn build(mut self) -> VfsResult<EntityTreePage> {
        // order matters!
        let top_level_nodes = self.process_body(Some(self.module), self.ast_sheet.top_level_asts());
        Ok(EntityTreePage {
            top_level_entities_idx_range: self.arena.alloc_batch(top_level_nodes),
            isolate_entities_idx_range: self.arena.alloc_batch(self.sporadic_entities),
            arena: self.arena,
        })
    }

    fn process_stmt(&mut self, ast_idx: AstIdx) {
        assert!(self
            .process(None, ast_idx, &self.ast_sheet[ast_idx])
            .is_none())
    }

    fn process(
        &mut self,
        parent: Option<EntityPath>,
        ast_idx: AstIdx,
        ast: &Ast,
    ) -> Option<EntityTree> {
        match ast {
            Ast::Err { .. } | Ast::Use { .. } | Ast::Comment { .. } | Ast::Decor { .. } => None,
            Ast::Stmt {
                token_group_idx,
                body,
            }
            | Ast::Main {
                token_group_idx,
                body,
            }
            | Ast::Config {
                token_group_idx,
                body,
            } => {
                let sporadic_entities = self.process_body(None, body);
                self.sporadic_entities.extend(sporadic_entities);
                None
            }
            Ast::IfElseStmts {
                if_stmt,
                elif_stmts,
                else_stmt,
            } => {
                self.process_stmt(*if_stmt);
                for elif_stmt in elif_stmts {
                    self.process_stmt(elif_stmt);
                }
                if let Some(else_stmt) = else_stmt {
                    self.process_stmt(*else_stmt)
                }
                None
            }
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
            } => {
                self.process_stmt(*pattern_stmt);
                for case_stmt in case_stmts {
                    self.process_stmt(case_stmt);
                }
                None
            }
            Ast::Defn {
                token_group_idx,
                body,
                accessibility,
                entity_card,
                ident,
                is_generic,
                body_kind,
            } => {
                let entity_path = self.db.it_entity_path(match parent {
                    Some(parent) => EntityPathData::Childpath {
                        parent,
                        ident: *ident,
                    },
                    None => {
                        p!(entity_card, ident.data(self.db));
                        todo!()
                    }
                });
                let subentities = self.process_body(
                    match entity_card {
                        EntityCard::Module | EntityCard::Type | EntityCard::Trait => {
                            Some(entity_path)
                        }
                        EntityCard::Form | EntityCard::EnumVariant | EntityCard::Use => None,
                    },
                    body,
                );
                Some(EntityTree {
                    node: EntityNode::new(entity_path, *accessibility, *entity_card),
                    ast_idx: Some(ast_idx),
                    subentities: self.arena.alloc_batch(subentities),
                })
            }
            Ast::Impl {
                token_group_idx,
                body,
                ..
            } => {
                for ast_idx in body {
                    let ast = &self.ast_sheet[ast_idx];
                    match ast {
                        Ast::Err { .. }
                        | Ast::Use { .. }
                        | Ast::Comment { .. }
                        | Ast::Decor { .. } => (),
                        Ast::Defn { body, .. } => {
                            let isolate_entities = self.process_body(None, body);
                            self.sporadic_entities.extend(isolate_entities)
                        }
                        Ast::Stmt { .. }
                        | Ast::IfElseStmts { .. }
                        | Ast::MatchStmts { .. }
                        | Ast::Impl { .. }
                        | Ast::Main { .. }
                        | Ast::Config { .. } => {
                            p!(self.debug_ast(ast));
                            todo!()
                        }
                    }
                }
                None
            }
        }
    }

    fn debug_ast(&self, ast: &Ast) {
        let token_sheet = self.db.token_sheet(self.module).as_ref().unwrap();
        match ast {
            Ast::Err { .. } => todo!(),
            Ast::Use {
                token_group_idx, ..
            } => todo!(),
            Ast::Comment { .. } => todo!(),
            Ast::Decor { .. } => todo!(),
            Ast::Stmt {
                token_group_idx,
                body,
            } => {
                p!(self.module.show(self.db), &token_sheet[*token_group_idx]);
                todo!()
            }
            Ast::IfElseStmts {
                if_stmt,
                elif_stmts,
                else_stmt,
            } => todo!(),
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
            } => todo!(),
            Ast::Defn {
                token_group_idx,
                body,
                accessibility,
                entity_card,
                ident,
                is_generic,
                body_kind,
            } => todo!(),
            Ast::Impl {
                token_group_idx,
                body,
            } => todo!(),
            Ast::Main {
                token_group_idx,
                body,
            } => todo!(),
            Ast::Config {
                token_group_idx,
                body,
            } => todo!(),
        }
    }

    fn process_body(&mut self, parent: Option<EntityPath>, body: &AstIdxRange) -> Vec<EntityTree> {
        body.into_iter()
            .filter_map(|idx| {
                let ast = &self.ast_sheet[idx];
                self.process(parent, idx, ast)
            })
            .collect()
    }
}
