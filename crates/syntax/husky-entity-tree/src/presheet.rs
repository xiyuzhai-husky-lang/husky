mod action;
mod entity_use_tracker;
mod use_all_tracker;

pub(crate) use action::*;
use husky_print_utils::p;

use crate::{ModuleItem, *};
use entity_use_tracker::*;
use husky_text::TextRange;
use use_all_tracker::*;
use vec_like::{AsVecMapEntry, InsertEntryRepeatError};

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_presheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> VfsResult<EntityTreePresheet> {
    Ok(EntitySymbolPresheetBuilder::new(db, module_path)?.build())
}

#[test]
fn entity_tree_presheet_works() {
    DB::expect_test_probable_modules_debug_ref_with_db("entity_tree_presheet", |db, module_path| {
        entity_tree_presheet(db, module_path)
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct EntityTreePresheet {
    module_path: ModulePath,
    module_symbols: VecMap<EntitySymbol>,
    entity_use_trackers: EntityUseExprTrackers,
    use_all_trackers: UseAllTrackers,
    errors: Vec<EntityTreeError>,
}

// impl Into<EntityTreeSheet> for EntityTreePresheet {
//     fn into(self) -> EntityTreeSheet {
//         EntityTreeSheet::new(self.module_path, self.module_symbols)
//     }
// }

impl EntityTreePresheet {
    pub(crate) fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub(crate) fn module_symbols(&self) -> &VecMap<EntitySymbol> {
        &self.module_symbols
    }

    pub(crate) fn into_sheet(self, impl_blocks: ImplBlockIdxRange) -> EntityTreeModuleSheet {
        EntityTreeModuleSheet::new(self.module_path, self.module_symbols, impl_blocks)
    }
}

impl AsVecMapEntry for EntityTreePresheet {
    type K = ModulePath;
    fn key(&self) -> ModulePath {
        self.module_path
    }

    fn key_ref(&self) -> &ModulePath {
        &self.module_path
    }
}

struct EntitySymbolPresheetBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    ast_sheet: &'a AstSheet,
    module_path: ModulePath,
    nodes: VecMap<EntitySymbol>,
    entity_use_trackers: EntityUseExprTrackers,
    errors: Vec<EntityTreeError>,
}

impl<'a> EntitySymbolPresheetBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            ast_sheet: db.ast_sheet(module_path)?,
            module_path,
            nodes: Default::default(),
            entity_use_trackers: Default::default(),
            errors: vec![],
        })
    }

    fn build(mut self) -> EntityTreePresheet {
        for (ast_idx, ast) in self.ast_sheet.all_ast_indexed_iter() {
            self.process(ast_idx, ast)
        }
        EntityTreePresheet {
            module_path: self.module_path,
            module_symbols: self.nodes,
            entity_use_trackers: self.entity_use_trackers,
            use_all_trackers: Default::default(),
            errors: self.errors,
        }
    }

    fn process(&mut self, ast_idx: AstIdx, ast: &Ast) {
        match ast {
            Ast::Use {
                token_group_idx,
                accessibility,
                ident,
                use_expr_idx,
            } => self.entity_use_trackers.push(EntityUseTracker::new_root(
                ast_idx,
                *accessibility,
                *ident,
                *use_expr_idx,
            )),
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
                let ident = ident_token.ident();
                if let Some(entity_path) = entity_path {
                    let new_node = match entity_path {
                        EntityPath::Module(module_path) => EntitySymbol::Submodule {
                            ident,
                            accessibility: *accessibility,
                            module_path: *module_path,
                            ast_idx,
                        },
                        EntityPath::ModuleItem(module_item_path) => EntitySymbol::ModuleItem(
                            ModuleItem::new(ident, *accessibility, *module_item_path, ast_idx),
                        ),
                        EntityPath::AssociatedItem(_) | EntityPath::Variant(_) => return,
                        EntityPath::GenericParameter(_) => todo!(),
                    };
                    match self.nodes.insert_new(new_node) {
                        Ok(_) => (),
                        Err(e) => {
                            let old_node = &self.nodes.data()[e.old];
                            self.errors
                                .push(EntityTreeError::EntitySymbolAlreadyDefined {
                                    old: old_node.ast_idx().unwrap(),
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

impl salsa::DebugWithDb<dyn EntityTreeDb + '_> for EntityTreePresheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityTreeDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        f.debug_struct("EntityTreePresheet")
            .field(
                "module_path",
                &self
                    .module_path
                    .debug_with(db as &dyn VfsDb, include_all_fields),
            )
            .field(
                "module_symbols",
                &(&self.module_symbols).debug_with(db, include_all_fields),
            )
            .field("entity_use_roots", &self.entity_use_trackers)
            .finish()
    }
}

impl<Db> salsa::DebugWithDb<Db> for EntityTreePresheet
where
    Db: EntityTreeDb,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityTreeDb, include_all_fields)
    }
}
