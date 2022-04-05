use std::sync::Arc;

use ast::AstKind;
use entity_route::{EntityRoutePtr, EntitySource};
use fold::{FoldIterItem, FoldStorage};
use infer_total::InferQueryGroup;
use semantics_lazy::parse_lazy_stmts;
use syntax_types::RoutineKind;
use upcast::Upcast;
use vm::InstructionSheet;

use crate::*;
use semantics_eager::parse_impr_stmts;
use semantics_error::*;

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityQueryGroup:
    InferQueryGroup + ast::AstQueryGroup + ControlEntityVersion + Upcast<dyn InferQueryGroup>
{
    fn main(&self, main_file: file::FilePtr) -> SemanticResultArc<Main>;
    fn entity(&self, scope: EntityRoutePtr) -> SemanticResult<Arc<Entity>>;
    fn subentities(&self, scope: EntityRoutePtr) -> SemanticResultArc<Vec<Arc<Entity>>>;
}

fn main(this: &dyn EntityQueryGroup, main_file: file::FilePtr) -> SemanticResultArc<Main> {
    let ast_text = this.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::MainDefn => {
                return Ok(Arc::new(Main {
                    stmts: parse_lazy_stmts(
                        &[],
                        this.upcast(),
                        &ast_text.arena,
                        not_none!(item.children),
                        main_file,
                    )?,
                    file: main_file,
                }))
            }
            _ => (),
        }
    }
    err!("main not found")
}

fn entity(db: &dyn EntityQueryGroup, entity_scope: EntityRoutePtr) -> SemanticResultArc<Entity> {
    let source = db.entity_source(entity_scope)?;
    match source {
        EntitySource::Builtin(_) => todo!(),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let arena = &ast_text.arena;
            let FoldIterItem {
                value, children, ..
            } = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap();
            let ast_head = value.as_ref()?;

            let (ident, entity_kind) = match ast_head.kind {
                AstKind::TypeDefnHead {
                    ident,
                    kind,
                    generic_placeholders: ref generics,
                } => {
                    let signature = try_infer!(db.ty_decl(entity_scope));
                    (
                        ident,
                        EntityKind::Ty(TyDefn::from_ast(
                            db.upcast(),
                            ast_head,
                            not_none!(children),
                            arena,
                            file,
                        )?),
                    )
                }
                AstKind::RoutineDefnHead {
                    ref routine_kind,
                    ref routine_head,
                } => (
                    routine_head.routine_name,
                    EntityKind::routine(
                        db,
                        routine_kind,
                        routine_head,
                        not_none!(children),
                        arena,
                        file,
                    )?,
                ),
                AstKind::PatternDefnHead => todo!(),
                AstKind::Use { ident, scope } => todo!(),
                AstKind::MainDefn | AstKind::DatasetConfigDefnHead | AstKind::Stmt(_) => panic!(),
                AstKind::EnumVariantDefnHead {
                    ident,
                    raw_variant_kind: ref variant_kind,
                } => todo!(),
                AstKind::MembVarDefn { .. } => todo!(),
                AstKind::MembRoutineDefnHead { .. } => todo!(),
                AstKind::FeatureDecl { ident, ty } => (
                    ident,
                    EntityKind::feature(db, ty, not_none!(children), arena, file)?,
                ),
                AstKind::MembFeatureDefnHead { ident, ty } => todo!(),
            };
            Ok(Arc::new(Entity::new(
                ident,
                entity_kind,
                Arc::new(Vec::new()),
                entity_scope,
                file,
                ast_head.range,
                db.entity_vc(),
            )))
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Contextual { .. } => todo!(),
    }
}

pub(crate) fn subentities(
    this: &dyn EntityQueryGroup,
    scope_id: EntityRoutePtr,
) -> SemanticResultArc<Vec<Arc<Entity>>> {
    this.subscopes(scope_id)
        .iter()
        .map(|scope| this.entity(*scope))
        .collect::<SemanticResult<Vec<Arc<Entity>>>>()
        .map(|v| Arc::new(v))
}
