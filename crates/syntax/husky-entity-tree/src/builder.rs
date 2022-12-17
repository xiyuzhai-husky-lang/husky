use crate::*;
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_print_utils::p;
use std::collections::HashSet;

pub struct EntityTreeBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    module: EntityPath,
    ast_sheet: &'a AstSheet,
    arena: EntityTreeArena,
    isolate_entities: Vec<EntityTree>,
}

impl<'a> EntityTreeBuilder<'a> {
    pub fn new(db: &'a dyn EntityTreeDb, module: EntityPath, ast_sheet: &'a AstSheet) -> Self {
        Self {
            db,
            module,
            ast_sheet,
            arena: Default::default(),
            isolate_entities: Default::default(),
        }
    }

    pub fn build(mut self) -> VfsResult<EntityTreeSheet> {
        // order matters!
        let top_level_nodes = self.process_body(Some(self.module), self.ast_sheet.top_level_asts());
        Ok(EntityTreeSheet {
            top_level_entities: self.arena.alloc_batch(top_level_nodes),
            isolate_entities: self.arena.alloc_batch(self.isolate_entities),
            arena: self.arena,
        })
    }

    fn process(
        &mut self,
        parent: Option<EntityPath>,
        ast_idx: AstIdx,
        ast: &Ast,
    ) -> Option<EntityTree> {
        match ast {
            Ast::Err(_, _) | Ast::Use { .. } | Ast::Comment(_) | Ast::Decor(_) => None,
            Ast::Stmt { token_group, body } => todo!(),
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
                token_group,
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
                        p!(ident.data(self.db));
                        todo!()
                    }
                });
                let subentities = self.process_body(Some(entity_path), body);
                Some(EntityTree {
                    node: EntityNode::new(entity_path, *accessibility, *entity_card),
                    ast_idx: Some(ast_idx),
                    subentities: self.arena.alloc_batch(subentities),
                })
            }
            Ast::Impl { body, .. } => {
                todo!()
                // let isolated_entities = self.process_body(None, body);
                // if isolated_entities.len() != 0 {
                //     todo!()
                // }
                // None
            }
            Ast::Main { token_group, body } => todo!(),
            Ast::Config { token_group, body } => todo!(),
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
