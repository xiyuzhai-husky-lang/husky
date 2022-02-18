mod ty;

use std::sync::Arc;

use ast::Ast;
use common::Upcast;
use file::FilePtr;
use fold::{FoldIterItem, FoldStorage};
use scope::ScopePtr;

use crate::{error::not_none, *};

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityQueryGroup:
    ast::AstQueryGroup + ControlEntityVersion + Upcast<dyn InferQueryGroup>
{
    fn entity(&self, scope: ScopePtr) -> SemanticResult<Arc<Entity>>;
    fn subentities(&self, scope: ScopePtr) -> SemanticResultArc<Vec<Arc<Entity>>>;
}

fn entity(this: &dyn EntityQueryGroup, scope: ScopePtr) -> SemanticResultArc<Entity> {
    let source = this.scope_source(scope)?;
    match source {
        scope::ScopeSource::Builtin(_) => todo!(),
        scope::ScopeSource::WithinBuiltinModule => todo!(),
        scope::ScopeSource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = this.ast_text(file)?;
            entity_from_ast(
                this,
                file,
                &ast_text,
                token_group_index,
                this.subentities(scope)?,
                scope,
                this.entity_vc(),
            )
        }
        scope::ScopeSource::Module { file: file_id } => todo!(),
    }
}

pub fn entity_from_ast(
    this: &dyn EntityQueryGroup,
    file: FilePtr,
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
        Ast::TypeDef {
            ident,
            kind,
            generics,
        } => ty::ty_from_ast(*ident, kind, generics, children, subentities, scope, vc),
        Ast::FuncDef { kind, decl } => {
            let kind = match kind {
                syntax_types::FuncKind::Test => todo!(),
                syntax_types::FuncKind::Proc => todo!(),
                syntax_types::FuncKind::Func => EntityKind::Func {
                    inputs: decl.inputs.clone(),
                    stmts: stmt::parse_decl_stmts(
                        this.upcast(),
                        &ast_text.arena,
                        not_none!(children),
                        file,
                    )?,
                },
                syntax_types::FuncKind::Def => todo!(),
            };
            Ok(Arc::new(Entity::new(
                decl.funcname,
                kind,
                Arc::new(Vec::new()),
                scope,
                vc,
            )))
        }
        Ast::PatternDef => todo!(),
        Ast::Use { ident, scope } => todo!(),
        Ast::MembDef { ident, kind } => todo!(),
        Ast::MainDef | Ast::DatasetConfig | Ast::Stmt(_) => panic!(),
    }
}

pub(crate) fn subentities(
    this: &dyn EntityQueryGroup,
    scope_id: ScopePtr,
) -> SemanticResultArc<Vec<Arc<Entity>>> {
    this.subscopes(scope_id)
        .iter()
        .map(|scope| this.entity(*scope))
        .collect::<SemanticResult<Vec<Arc<Entity>>>>()
        .map(|v| Arc::new(v))
}
