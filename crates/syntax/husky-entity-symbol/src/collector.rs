mod action;

use crate::*;
use husky_print_utils::p;
use vec_like::{AsVecMapEntry, VecMap};

pub(crate) struct EntitySymbolCollector<'a> {
    db: &'a dyn EntitySymbolDb,
    presheets: VecMap<ModulePath, &'a EntitySymbolPresheet>,
}

impl<'a> EntitySymbolCollector<'a> {
    pub(crate) fn new(db: &'a dyn EntitySymbolDb, crate_path: CratePath) -> VfsResult<Self> {
        let all_modules = db.all_modules_within_crate(crate_path)?;
        let presheets = all_modules
            .into_iter()
            .map(|module_path| Ok(entity_tree_presheet(db, *module_path).as_ref()?))
            .collect::<VfsResult<VecMap<ModulePath, &'a EntitySymbolPresheet>>>()?;
        Ok(Self { db, presheets })
    }

    pub(crate) fn collect_all(mut self) -> EntitySymbolBundle {
        loop {
            let actions = self.collect_possible_actions();
            if actions.len() == 0 {
                break;
            }
            todo!()
        }
        todo!()
        // order matters!
        //     let top_level_nodes = self.process_body(
        //         Some(self.module_entity_path),
        //         self.ast_sheet.top_level_asts(),
        //     );
        //     Ok(EntitySymbolSheet {
        //         top_level_entities_idx_range: self.arena.alloc_batch(top_level_nodes),
        //         isolate_entities_idx_range: self.arena.alloc_batch(self.sporadic_entities),
        //         arena: self.arena,
        //     })
    }

    // fn process_stmt(&mut self, ast_idx: AstIdx) {
    //     assert!(self
    //         .process(None, ast_idx, &self.ast_sheet[ast_idx])
    //         .is_none())
    // }

    // fn process(
    //     &mut self,
    //     parent: Option<EntityPath>,
    //     ast_idx: AstIdx,
    //     ast: &Ast,
    // ) -> Option<EntitySymbol> {
    //     match ast {
    //         Ast::Err { .. } | Ast::Use { .. } | Ast::Comment { .. } | Ast::Decor { .. } => None,
    //         Ast::Stmt {
    //             token_group_idx: _,
    //             body,
    //         }
    //         | Ast::Main {
    //             token_group_idx: _,
    //             body,
    //         }
    //         | Ast::Config {
    //             token_group_idx: _,
    //             body,
    //         } => {
    //             todo!()
    //             // let sporadic_entities = self.process_body(None, body);
    //             // self.sporadic_entities.extend(sporadic_entities);
    //             // None
    //         }
    //         Ast::IfElseStmts {
    //             if_stmt,
    //             elif_stmts,
    //             else_stmt,
    //         } => {
    //             self.process_stmt(*if_stmt);
    //             for elif_stmt in elif_stmts {
    //                 self.process_stmt(elif_stmt);
    //             }
    //             if let Some(else_stmt) = else_stmt {
    //                 self.process_stmt(*else_stmt)
    //             }
    //             None
    //         }
    //         Ast::MatchStmts {
    //             pattern_stmt,
    //             case_stmts,
    //         } => {
    //             self.process_stmt(*pattern_stmt);
    //             for case_stmt in case_stmts {
    //                 self.process_stmt(case_stmt);
    //             }
    //             None
    //         }
    //         Ast::Defn {
    //             token_group_idx: _,
    //             body,
    //             accessibility,
    //             entity_kind,
    //             entity_path,
    //             ident,
    //             is_generic: _,
    //             body_kind: _,
    //         } => {
    //             todo!();
    //             // let subentities = self.process_body(
    //             //     match entity_kind {
    //             //         EntityKind::Module | EntityKind::Type | EntityKind::Trait => {
    //             //             Some(entity_path)
    //             //         }
    //             //         EntityKind::Form | EntityKind::EnumVariant | EntityKind::Use => None,
    //             //     },
    //             //     body,
    //             // );
    //             todo!()
    //             // Some(EntitySymbol {
    //             //     node: EntityNode::new(entity_path, *accessibility, *entity_kind),
    //             //     ast_idx: Some(ast_idx),
    //             //     subentities: self.arena.alloc_batch(subentities),
    //             // })
    //         }
    //         Ast::Impl { body, .. } => {
    //             for ast_idx in body {
    //                 let ast = &self.ast_sheet[ast_idx];
    //                 match ast {
    //                     Ast::Err { .. }
    //                     | Ast::Use { .. }
    //                     | Ast::Comment { .. }
    //                     | Ast::Decor { .. } => (),
    //                     Ast::Defn { body, .. } => {
    //                         let isolate_entities = self.process_body(None, body);
    //                         todo!()
    //                         // self.sporadic_entities.extend(isolate_entities)
    //                     }
    //                     Ast::Stmt { .. }
    //                     | Ast::IfElseStmts { .. }
    //                     | Ast::MatchStmts { .. }
    //                     | Ast::Impl { .. }
    //                     | Ast::Main { .. }
    //                     | Ast::Config { .. } => {
    //                         p!(self.debug_ast(ast));
    //                         todo!()
    //                     }
    //                     Ast::ModuleItemVariant { .. } => todo!(),
    //                 }
    //             }
    //             None
    //         }
    //         Ast::ModuleItemVariant { .. } => todo!(),
    //     }
    // }

    // fn process_body(&mut self, parent: Option<EntityPath>, body: &AstIdxRange) -> Vec<EntitySymbol> {
    //     body.into_iter()
    //         .filter_map(|idx| {
    //             let ast = &self.ast_sheet[idx];
    //             self.process(parent, idx, ast)
    //         })
    //         .collect()
    // }
}
