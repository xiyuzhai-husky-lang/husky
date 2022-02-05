mod ty;

use std::sync::Arc;

use fold::{FoldIterItem, FoldStorage};
use scope::ScopePtr;

use crate::*;

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityQueryGroup: ast::AstQueryGroup + ControlEntityVersion {
    fn entity(&self, scope: ScopePtr) -> SemanticResult<Arc<Entity>>;
    fn subentities(&self, scope: ScopePtr) -> SemanticResultArc<Vec<Arc<Entity>>>;
}

fn entity(this: &dyn EntityQueryGroup, scope: ScopePtr) -> SemanticResultArc<Entity> {
    let source = this.scope_source(scope)?;
    match source {
        scope::ScopeSource::Builtin(_) => todo!(),
        scope::ScopeSource::WithinBuiltinModule => todo!(),
        scope::ScopeSource::WithinModule {
            file_id,
            token_group_index,
        } => {
            let ast_text = this.ast_text(file_id)?;
            entity_from_ast(
                &ast_text,
                token_group_index,
                this.subentities(scope)?,
                scope,
                this.entity_vc(),
            )
        }
        scope::ScopeSource::Module { file_id } => todo!(),
    }
}

pub fn entity_from_ast(
    ast_text: &ast::AstText,
    token_group_index: usize,
    subentities: Arc<Vec<Arc<Entity>>>,
    scope: ScopePtr,
    vc: &EntityVersionControl,
) -> SemanticResultArc<Entity> {
    let FoldIterItem {
        value, children, ..
    } = ast_text
        .folded_results
        .fold_iter(token_group_index)
        .next()
        .unwrap();
    let head = value.as_ref()?;
    match head {
        ast::Ast::TypeDef {
            ident,
            kind,
            generics,
        } => ty::ty_from_ast(*ident, kind, generics, children, subentities, scope, vc),
        ast::Ast::FuncDef { kind, decl } => todo!(),
        ast::Ast::PatternDef => todo!(),
        ast::Ast::Use { ident, scope } => todo!(),
        ast::Ast::MembDef { ident, kind } => todo!(),
        ast::Ast::MainDef | ast::Ast::DatasetConfig | ast::Ast::Stmt(_) => panic!(),
    }
}

pub(crate) fn subentities(
    this: &dyn EntityQueryGroup,
    scope_id: ScopePtr,
) -> SemanticResultArc<Vec<Arc<Entity>>> {
    this.subscope_ids(scope_id)
        .iter()
        .map(|scope| this.entity(*scope))
        .collect::<SemanticResult<Vec<Arc<Entity>>>>()
        .map(|v| Arc::new(v))
}
