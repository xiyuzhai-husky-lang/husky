use crate::*;
use ast::AstKind;
use entity_route::EntityRoutePtr;
use entity_route::EntitySource;
use fold::{FoldIterItem, FoldStorage};
use semantics_error::*;
use semantics_lazy::parse_lazy_stmts;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use word::{CustomIdentifier, Identifier};

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

pub(crate) fn main_defn(
    this: &dyn EntityQueryGroup,
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

pub(crate) fn opt_entity_defn(
    db: &dyn EntityQueryGroup,
    entity_scope: EntityRoutePtr,
) -> SemanticResult<Option<Arc<EntityDefn>>> {
    let source = db.entity_source(entity_scope)?;
    match source {
        EntitySource::Builtin(_) => Ok(None),
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
                    EntityDefnVariant::enum_variant(db, variant_class, children),
                ),
                AstKind::MembVarDefn { .. } => todo!(),
                AstKind::MembRoutineDefnHead { .. } => todo!(),
                AstKind::FeatureDecl { ident, ty } => (
                    ident,
                    EntityDefnVariant::feature(db, ty, not_none!(children), arena, file)?,
                ),
                AstKind::MembFeatureDefnHead { ident, ty } => todo!(),
            };
            Ok(Some(Arc::new(EntityDefn::new(
                ident.into(),
                entity_kind,
                Arc::new(Vec::new()),
                entity_scope,
                file,
                ast_head.range,
            ))))
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Contextual { .. } => todo!(),
    }
}

pub(crate) fn subentity_defns(
    this: &dyn EntityQueryGroup,
    scope_id: EntityRoutePtr,
) -> SemanticResultArc<Vec<Arc<EntityDefn>>> {
    let mut defns = Vec::new();
    for opt_defn_result in this
        .subscopes(scope_id)
        .iter()
        .map(|scope| this.opt_entity_defn(*scope))
    {
        let opt_defn = opt_defn_result?;
        if let Some(defn) = opt_defn {
            defns.push(defn);
        }
    }
    Ok(Arc::new(defns))
}

pub(crate) fn entity_defn_uid(
    db: &dyn EntityQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntityDefnUid {
    let _defn = db.opt_entity_defn(entity_route);
    EntityDefnUid::new()
}
