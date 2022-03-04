mod ty;

use std::sync::Arc;

use ast::{Ast, AstKind};
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
    match head.kind {
        AstKind::TypeDef {
            ident,
            ref kind,
            ref generics,
        } => ty::ty_from_ast(
            ident,
            kind,
            generics,
            children,
            subentities,
            scope,
            file,
            head.range,
            vc,
        ),
        AstKind::RoutineDef { ref kind, ref decl } => {
            let kind = match kind {
                syntax_types::RoutineKind::Test => todo!(),
                syntax_types::RoutineKind::Proc => EntityKind::Proc {
                    input_placeholders: decl.input_placeholders.clone(),
                    output: decl.output,
                    stmts: stmt::parse_impr_stmts(
                        this.upcast(),
                        &ast_text.arena,
                        not_none!(children),
                        file,
                    )?,
                },
                syntax_types::RoutineKind::Func => EntityKind::Func {
                    input_placeholders: decl.input_placeholders.clone(),
                    output: decl.output,
                    stmts: stmt::parse_decl_stmts(
                        this.upcast(),
                        &ast_text.arena,
                        not_none!(children),
                        file,
                    )?,
                },
                syntax_types::RoutineKind::Def => todo!(),
            };
            Ok(Arc::new(Entity::new(
                decl.funcname,
                kind,
                Arc::new(Vec::new()),
                scope,
                file,
                head.range,
                vc,
            )))
        }
        AstKind::PatternDef => todo!(),
        AstKind::Use { ident, scope } => todo!(),
        AstKind::MembDef { ident, ref kind } => todo!(),
        AstKind::MainDef | AstKind::DatasetConfig | AstKind::Stmt(_) => panic!(),
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
