mod dependence;
mod morphism;
mod query;
mod routine;
mod trai;
mod ty;

pub use morphism::*;
pub use query::*;
pub use routine::*;
pub use trai::*;
pub use ty::*;

use ast::AstKind;
use defn_head::*;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use entity_route::{EntitySource, StaticEntityDefn};
use entity_syntax::*;
use file::FilePtr;
use fold::{FoldIterItem, FoldStorage};
use semantics_eager::*;
use semantics_error::*;
use semantics_lazy::parse_lazy_stmts;
use semantics_lazy::{LazyExpr, LazyExprKind, LazyOpnKind, LazyStmt, LazyStmtKind};
use std::sync::Arc;
use std::{
    f32::consts::E,
    sync::atomic::{AtomicUsize, Ordering},
};
use text::TextRange;
use word::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct EntityDefnUid {
    raw: usize,
}

static ENTITY_DEFN_NEXT_RAW_ID: AtomicUsize = AtomicUsize::new(0);

impl EntityDefnUid {
    pub fn new() -> EntityDefnUid {
        let raw = ENTITY_DEFN_NEXT_RAW_ID.fetch_add(1, Ordering::Relaxed);
        EntityDefnUid { raw }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityDefn {
    pub ident: Identifier,
    pub kind: Arc<EntityDefnVariant>,
    pub subentities: Arc<Vec<Arc<EntityDefn>>>,
    pub scope: EntityRoutePtr,
    pub file: FilePtr,
    pub range: TextRange,
}

impl EntityDefn {
    pub fn from_static(static_entity_defn: &StaticEntityDefn) -> Self {
        todo!()
    }

    pub fn kind(&self) -> &EntityDefnVariant {
        &self.kind
    }

    pub(crate) fn new(
        ident: Identifier,
        kind: EntityDefnVariant,
        subentities: Arc<Vec<Arc<EntityDefn>>>,
        scope: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    ) -> EntityDefn {
        let kind = Arc::new(kind);
        Self {
            ident,
            kind,
            subentities,
            scope,
            file,
            range,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityDefnVariant {
    Main(MainDefn),
    Module {},
    Feature {
        ty: RangedEntityRoute,
        lazy_stmts: Arc<Vec<Arc<LazyStmt>>>,
    },
    Pattern {},
    Func {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedEntityRoute,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    Proc {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedEntityRoute,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
    Ty(Arc<TyDefn>),
    EnumVariant(EnumVariantDefn),
    Builtin,
}

pub(crate) fn main_defn(
    this: &dyn EntityDefnQueryGroup,
    main_file: file::FilePtr,
) -> SemanticResultArc<MainDefn> {
    let ast_text = this.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::MainDefn => {
                return Ok(Arc::new(MainDefn {
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

pub(crate) fn entity_defn(
    db: &dyn EntityDefnQueryGroup,
    entity_scope: EntityRoutePtr,
) -> SemanticResultArc<EntityDefn> {
    let source = db.entity_source(entity_scope)?;
    match source {
        EntitySource::Static(static_entity_defn) => {
            Ok(Arc::new(EntityDefn::from_static(static_entity_defn)))
        }
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
                        EntityDefnVariant::Ty(TyDefn::from_ast(
                            db.upcast(),
                            ast_head,
                            not_none!(children),
                            arena,
                            file,
                        )?),
                    )
                }
                AstKind::RoutineDefnHead(ref head) => (
                    head.ident,
                    EntityDefnVariant::routine(db, head, not_none!(children), arena, file)?,
                ),
                AstKind::PatternDefnHead => todo!(),
                AstKind::Use { ident, scope } => todo!(),
                AstKind::MainDefn | AstKind::DatasetConfigDefnHead | AstKind::Stmt(_) => panic!(),
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => (
                    ident,
                    EntityDefnVariant::enum_variant(db, ident, variant_class, children),
                ),
                AstKind::FieldDefn { .. } => todo!(),
                AstKind::MethodDefnHead { .. } => todo!(),
                AstKind::FeatureDecl { ident, ty } => (
                    ident,
                    EntityDefnVariant::feature(db, ty, not_none!(children), arena, file)?,
                ),
                AstKind::MembFeatureDefnHead { ident, ty } => todo!(),
            };
            Ok(Arc::new(EntityDefn::new(
                ident.into(),
                entity_kind,
                todo!(),
                entity_scope,
                file,
                ast_head.range,
            )))
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Input { .. } => todo!(),
    }
}

pub(crate) fn subentity_defns(
    this: &dyn EntityDefnQueryGroup,
    scope_id: EntityRoutePtr,
) -> SemanticResultArc<Vec<Arc<EntityDefn>>> {
    let mut defns = Vec::new();
    for defn_result in this
        .subscopes(scope_id)
        .iter()
        .map(|scope| this.entity_defn(*scope))
    {
        let defn = defn_result?;
        defns.push(defn)
    }
    Ok(Arc::new(defns))
}

pub(crate) fn entity_defn_uid(
    db: &dyn EntityDefnQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntityDefnUid {
    let _defn = db.entity_defn(entity_route);
    EntityDefnUid::new()
}
