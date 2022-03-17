mod ty;

use std::sync::Arc;

use ast::{Ast, AstKind};
use common::Upcast;
use file::FilePtr;
use fold::{FoldIterItem, FoldStorage};
use scope::ScopePtr;
use semantics_lazy::parse_lazy_stmts;
use syntax_types::RoutineKind;
use vm::InstructionSheet;

use crate::*;
use semantics_eager::parse_impr_stmts;
use semantics_error::*;
use semantics_infer::*;

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityQueryGroup:
    ast::AstQueryGroup + ControlEntityVersion + Upcast<dyn InferQueryGroup>
{
    fn main(&self, main_file: file::FilePtr) -> SemanticResultArc<Main>;
    fn entity(&self, scope: ScopePtr) -> SemanticResult<Arc<Entity>>;
    fn subentities(&self, scope: ScopePtr) -> SemanticResultArc<Vec<Arc<Entity>>>;
    fn instruction_sheet(&self, scope: ScopePtr) -> SemanticResultArc<InstructionSheet>;
}

fn main(this: &dyn EntityQueryGroup, main_file: file::FilePtr) -> SemanticResultArc<Main> {
    let ast_text = this.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::MainDef => {
                return Ok(Arc::new(Main {
                    stmts: parse_lazy_stmts(
                        &[],
                        this.upcast(),
                        &ast_text.arena,
                        not_none!(item.children),
                        main_file,
                    )?,
                }))
            }
            _ => (),
        }
    }
    err!("main not found")
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
    db: &dyn EntityQueryGroup,
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
    let ast_head = value.as_ref()?;
    match ast_head.kind {
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
            ast_head.range,
            vc,
        ),
        AstKind::RoutineDef {
            ref routine_kind,
            ref routine_head,
        } => {
            let entity_kind = match routine_kind {
                RoutineKind::Test => todo!(),
                RoutineKind::Proc => EntityKind::Proc {
                    input_placeholders: routine_head.input_placeholders.clone(),
                    output: routine_head.output,
                    stmts: parse_impr_stmts(
                        &routine_head.input_placeholders,
                        db.upcast(),
                        &ast_text.arena,
                        not_none!(children),
                        file,
                    )?,
                },
                RoutineKind::Func => EntityKind::Func {
                    input_placeholders: routine_head.input_placeholders.clone(),
                    output: routine_head.output,
                    stmts: parse_decl_stmts(
                        &routine_head.input_placeholders,
                        db.upcast(),
                        &ast_text.arena,
                        not_none!(children),
                        file,
                    )?,
                },
                RoutineKind::Def => todo!(),
            };
            Ok(Arc::new(Entity::new(
                routine_head.funcname,
                entity_kind,
                Arc::new(Vec::new()),
                scope,
                file,
                ast_head.range,
                vc,
            )))
        }
        AstKind::PatternDef => todo!(),
        AstKind::Use { ident, scope } => todo!(),
        AstKind::MembDef {
            ident,
            memb_kind: ref kind,
        } => todo!(),
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

fn instruction_sheet(
    this: &dyn EntityQueryGroup,
    scope: ScopePtr,
) -> SemanticResultArc<InstructionSheet> {
    let entity = this.entity(scope)?;
    Ok(match entity.kind() {
        EntityKind::Module(_) => todo!(),
        EntityKind::Feature(_) => todo!(),
        EntityKind::Pattern(_) => todo!(),
        EntityKind::Func {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_decl(
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
        ),
        EntityKind::Proc {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_impr(
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
        ),
        EntityKind::Ty(_) => todo!(),
        EntityKind::Main(_) => todo!(),
    })
}
