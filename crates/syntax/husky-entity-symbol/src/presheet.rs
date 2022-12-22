mod action;
mod entity_use_tracker;
mod use_all_tracker;

pub(crate) use action::*;

use crate::*;
use entity_use_tracker::*;
use husky_text::TextRange;
use use_all_tracker::UseAllTracker;
use vec_like::AsVecMapEntry;

#[salsa::tracked(jar = EntitySymbolJar, return_ref)]
pub(crate) fn entity_tree_presheet(
    db: &dyn EntitySymbolDb,
    module_path: ModulePath,
) -> VfsResult<EntitySymbolPresheet> {
    Ok(EntitySymbolPresheetBuilder::new(db, module_path)?.build())
}

#[test]
fn entity_tree_presheet_works() {
    DB::expect_test_probable_modules_debug_with_db("entity_tree_presheet", |db, module_path| {
        entity_tree_presheet(db, module_path)
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct EntitySymbolPresheet {
    module_path: ModulePath,
    module_items: Vec<ModuleSymbol>,
    entity_use_trackers: Vec<EntityUseExprTracker>,
    use_all_trackers: Vec<UseAllTracker>,
}

impl AsVecMapEntry<ModulePath> for EntitySymbolPresheet {
    fn key(&self) -> ModulePath
    where
        ModulePath: Copy,
    {
        self.module_path
    }

    fn key_ref(&self) -> &ModulePath {
        &self.module_path
    }
}

struct EntitySymbolPresheetBuilder<'a> {
    db: &'a dyn EntitySymbolDb,
    ast_sheet: &'a AstSheet,
    module_path: ModulePath,
    module_symbols: Vec<ModuleSymbol>,
    entity_use_trackers: Vec<EntityUseExprTracker>,
}

impl<'a> EntitySymbolPresheetBuilder<'a> {
    fn new(db: &'a dyn EntitySymbolDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            ast_sheet: db.ast_sheet(module_path)?,
            module_path,
            module_symbols: vec![],
            entity_use_trackers: vec![],
        })
    }

    fn build(mut self) -> EntitySymbolPresheet {
        for (ast_idx, ast) in self.ast_sheet.indexed_asts() {
            self.process(ast_idx, ast)
        }
        EntitySymbolPresheet {
            module_path: self.module_path,
            module_items: self.module_symbols,
            entity_use_trackers: self.entity_use_trackers,
            use_all_trackers: vec![],
        }
    }

    fn process(&mut self, ast_idx: AstIdx, ast: &Ast) {
        match ast {
            Ast::Use {
                token_group_idx,
                accessibility,
                ident,
                use_expr_idx,
            } => self
                .entity_use_trackers
                .push(EntityUseExprTracker::new_root(
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
                ident,
                is_generic,
                body_kind,
            } => match entity_path {
                EntityPath::Module(_) => self
                    .module_symbols
                    .push(ModuleSymbol::Submodule { ident: *ident }),
                EntityPath::ModuleItem(module_item_path) => {
                    self.module_symbols.push(ModuleSymbol::ModuleItem {
                        ident: *ident,
                        ast_idx,
                        path: *module_item_path,
                        variants: match body_kind {
                            DefnBodyKind::None | DefnBodyKind::Block => None,
                            DefnBodyKind::EnumVariants => Some(
                                body.into_iter()
                                    .map(|variant_ast_idx| match self.ast_sheet[variant_ast_idx] {
                                        Ast::ModuleItemVariant {
                                            token_group_idx,
                                            module_item_variant_path,
                                            ident,
                                        } => ModuleItemVariant::new(ident, variant_ast_idx),
                                        _ => unreachable!(),
                                    })
                                    .collect(),
                            ),
                            DefnBodyKind::MatchCases => unreachable!(),
                        },
                    })
                }
                EntityPath::AssociatedItem(_) => (),
                EntityPath::EnumVariant(_) => (),
            },
            Ast::Err { .. }
            | Ast::Comment { .. }
            | Ast::Decor { .. }
            | Ast::Stmt { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. }
            | Ast::ModuleItemVariant { .. }
            | Ast::Impl { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => (),
        }
    }
}

impl salsa::DebugWithDb<dyn EntitySymbolDb + '_> for EntitySymbolPresheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntitySymbolDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        f.debug_struct("EntitySymbolPresheet")
            .field(
                "module_path",
                &self
                    .module_path
                    .debug_with(db as &dyn VfsDb, include_all_fields),
            )
            .field(
                "module_items",
                &self.module_items.debug_with(db, include_all_fields),
            )
            .field("entity_use_roots", &self.entity_use_trackers)
            .finish()
    }
}

impl<Db> salsa::DebugWithDb<Db> for EntitySymbolPresheet
where
    Db: EntitySymbolDb,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntitySymbolDb, include_all_fields)
    }
}
