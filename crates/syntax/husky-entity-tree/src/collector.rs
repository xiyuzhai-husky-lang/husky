mod state;

use crate::*;
use husky_print_utils::p;
use vec_like::{AsVecMapEntry, VecMap};

pub(crate) struct EntityTreeCollector<'a> {
    db: &'a dyn EntityTreeDb,
    ast_sheet: &'a AstSheet,
    presheets: VecMap<ModulePath, EntityTreePresheet>,
}

impl<'a> EntityTreeCollector<'a> {
    pub(crate) fn new(
        db: &'a dyn EntityTreeDb,
        module_path: ModulePath,
        ast_sheet: &'a AstSheet,
    ) -> Self {
        todo!()
        // let module_entity_path = EntityPath::new_module(db, module_path);
        // Self {
        //     db,
        //     module_path,
        //     module_entity_path,
        //     ast_sheet,
        //     arena: Default::default(),
        //     sporadic_entities: Default::default(),
        // }
    }

    pub(crate) fn collect_all(mut self) -> VfsResult<EntityTreeSheet> {
        todo!()
        // order matters!
        //     let top_level_nodes = self.process_body(
        //         Some(self.module_entity_path),
        //         self.ast_sheet.top_level_asts(),
        //     );
        //     Ok(EntityTreeSheet {
        //         top_level_entities_idx_range: self.arena.alloc_batch(top_level_nodes),
        //         isolate_entities_idx_range: self.arena.alloc_batch(self.sporadic_entities),
        //         arena: self.arena,
        //     })
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
                token_group_idx: _,
                body,
            }
            | Ast::Main {
                token_group_idx: _,
                body,
            }
            | Ast::Config {
                token_group_idx: _,
                body,
            } => {
                todo!()
                // let sporadic_entities = self.process_body(None, body);
                // self.sporadic_entities.extend(sporadic_entities);
                // None
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
                token_group_idx: _,
                body,
                accessibility,
                entity_kind,
                entity_path,
                ident,
                is_generic: _,
                body_kind: _,
            } => {
                todo!();
                // let subentities = self.process_body(
                //     match entity_kind {
                //         EntityKind::Module | EntityKind::Type | EntityKind::Trait => {
                //             Some(entity_path)
                //         }
                //         EntityKind::Form | EntityKind::EnumVariant | EntityKind::Use => None,
                //     },
                //     body,
                // );
                todo!()
                // Some(EntityTree {
                //     node: EntityNode::new(entity_path, *accessibility, *entity_kind),
                //     ast_idx: Some(ast_idx),
                //     subentities: self.arena.alloc_batch(subentities),
                // })
            }
            Ast::Impl { body, .. } => {
                for ast_idx in body {
                    let ast = &self.ast_sheet[ast_idx];
                    match ast {
                        Ast::Err { .. }
                        | Ast::Use { .. }
                        | Ast::Comment { .. }
                        | Ast::Decor { .. } => (),
                        Ast::Defn { body, .. } => {
                            let isolate_entities = self.process_body(None, body);
                            todo!()
                            // self.sporadic_entities.extend(isolate_entities)
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
                        Ast::ModuleItemVariant { .. } => todo!(),
                    }
                }
                None
            }
            Ast::ModuleItemVariant { .. } => todo!(),
        }
    }

    fn debug_ast(&self, ast: &Ast) {
        todo!()
        // let token_sheet = self.db.token_sheet(self.module_path).as_ref().unwrap();
        // match ast {
        //     Ast::Err { .. } => todo!(),
        //     Ast::Use { .. } => todo!(),
        //     Ast::Comment { .. } => todo!(),
        //     Ast::Decor { .. } => todo!(),
        //     Ast::Stmt {
        //         token_group_idx,
        //         body: _,
        //     } => {
        //         todo!()
        //     }
        //     Ast::IfElseStmts {
        //         if_stmt: _,
        //         elif_stmts: _,
        //         else_stmt: _,
        //     } => todo!(),
        //     Ast::MatchStmts {
        //         pattern_stmt: _,
        //         case_stmts: _,
        //     } => todo!(),
        //     Ast::Defn {
        //         token_group_idx: _,
        //         body: _,
        //         accessibility: _,
        //         entity_kind: _,
        //         ident: _,
        //         is_generic: _,
        //         body_kind: _,
        //     } => todo!(),
        //     Ast::Impl {
        //         token_group_idx: _,
        //         body: _,
        //     } => todo!(),
        //     Ast::Main {
        //         token_group_idx: _,
        //         body: _,
        //     } => todo!(),
        //     Ast::Config {
        //         token_group_idx: _,
        //         body: _,
        //     } => todo!(),
        // }
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
