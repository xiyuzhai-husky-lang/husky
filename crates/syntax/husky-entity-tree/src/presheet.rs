use crate::*;
use husky_text::TextRange;
use vec_like::AsVecMapEntry;

#[salsa::tracked(jar = EntityTreeJar )]
pub(crate) fn entity_tree_presheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> VfsResult<EntityTreePresheet> {
    Ok(EntityTreePresheetBuilder::new(db, module_path)?.build())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct EntityTreePresheet {
    module_path: ModulePath,
    module_items: Vec<ModuleItem>,
    entity_uses: Vec<EntityUse>,
    use_all_progresses: Vec<UseAllProgress>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct UseAllProgress {}

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
    module_path: ModulePath,
    ast_sheet: &'a AstSheet,
}

impl<'a> EntityTreePresheetBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            module_path,
            ast_sheet: db.ast_sheet(module_path)?,
        })
    }

    fn build(mut self) -> EntityTreePresheet {
        todo!()
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
