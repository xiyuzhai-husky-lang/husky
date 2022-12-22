mod action;
mod entity_use;
mod use_all;

pub(crate) use action::*;

use crate::*;
use entity_use::*;
use husky_text::TextRange;
use use_all::UseAllTracker;
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
    DB::expect_test_probable_modules_debug_with_db("entity_tree_presheet", |db, module_path| {
        entity_tree_presheet(db, module_path)
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct EntityTreePresheet {
    module_path: ModulePath,
    module_items: Vec<ModuleSymbol>,
    entity_use_trackers: Vec<EntityUseExprTracker>,
    use_all_trackers: Vec<UseAllTracker>,
}

impl AsVecMapEntry<ModulePath> for EntityTreePresheet {
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

struct EntityTreePresheetBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    ast_sheet: &'a AstSheet,
    module_path: ModulePath,
    module_symbols: Vec<ModuleSymbol>,
    entity_use_trackers: Vec<EntityUseExprTracker>,
}

impl<'a> EntityTreePresheetBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            ast_sheet: db.ast_sheet(module_path)?,
            module_path,
            module_symbols: vec![],
            entity_use_trackers: vec![],
        })
    }

    fn build(mut self) -> EntityTreePresheet {
        for (ast_idx, ast) in self.ast_sheet.indexed_asts() {
            self.process(ast_idx, ast)
        }
        EntityTreePresheet {
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
                use_expr_idx,
            } => self
                .entity_use_trackers
                .push(EntityUseExprTracker::new_root(
                    ast_idx,
                    *accessibility,
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

enum BuilderContext {
    ExpectConnectedModuleItem,
    ExpectAssociatedItem { range: TextRange },
    ExpectDisconnectedModuleItem { range: TextRange },
}

impl BuilderContext {
    fn subcontext(&self, ast: &Ast, subrange: TextRange) -> Self {
        match self {
            BuilderContext::ExpectConnectedModuleItem => match ast {
                Ast::Err {
                    token_group_idx,
                    error,
                } => todo!(),
                Ast::Use {
                    token_group_idx,
                    accessibility,
                    use_expr_idx,
                } => todo!(),
                Ast::Comment { token_group_idx } => todo!(),
                Ast::Decor { token_group_idx } => todo!(),
                Ast::Stmt {
                    token_group_idx,
                    body,
                } => todo!(),
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
                    entity_kind,
                    entity_path,
                    ident,
                    is_generic,
                    body_kind,
                } => {
                    match entity_kind {
                        EntityKind::Module => todo!(),
                        EntityKind::ModuleItem(_) => todo!(),
                        EntityKind::EnumVariant => todo!(),
                    }
                    BuilderContext::ExpectAssociatedItem { range: subrange }
                }
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
                Ast::ModuleItemVariant { .. } => todo!(),
            },
            BuilderContext::ExpectAssociatedItem { .. } => {
                BuilderContext::ExpectDisconnectedModuleItem { range: subrange }
            }
            BuilderContext::ExpectDisconnectedModuleItem { .. } => {
                BuilderContext::ExpectAssociatedItem { range: subrange }
            }
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
                "module_items",
                &self.module_items.debug_with(db, include_all_fields),
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
