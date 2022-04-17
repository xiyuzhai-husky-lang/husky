mod dependence;
mod morphism;
mod query;
mod routine;
mod subentities;
mod trai;
mod ty;

use avec::Avec;
pub use morphism::*;
pub use query::*;
pub use routine::*;
use static_defn::StaticEntityDefn;
pub use trai::*;
pub use ty::*;

use ast::AstKind;
use defn_head::*;
use entity_kind::*;
use entity_route::{EntityRouteKind, EntitySource};
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use file::FilePtr;
use fold::{FoldIterItem, FoldStorage};
use semantics_eager::*;
use semantics_error::*;
use semantics_lazy::parse_lazy_stmts;
use semantics_lazy::{LazyExpr, LazyExprKind, LazyOpnKind, LazyStmt, LazyStmtKind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use text::TextRange;
use vec_dict::HasKey;
use vm::{FieldContract, InputContract};
use word::{CustomIdentifier, IdentDict, Identifier};

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

#[derive(Debug, PartialEq, Eq)]
pub struct EntityDefn {
    pub ident: Identifier,
    pub variant: EntityDefnVariant,
    pub subentities: Arc<Vec<Arc<EntityDefn>>>,
    pub route: EntityRoutePtr,
    pub file: FilePtr,
    pub range: TextRange,
}

impl HasKey<CustomIdentifier> for EntityDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident.custom()
    }
}

impl EntityDefn {
    pub fn from_static(static_entity_defn: &StaticEntityDefn) -> Self {
        todo!()
    }

    pub fn kind(&self) -> &EntityDefnVariant {
        &self.variant
    }

    pub(crate) fn new(
        ident: Identifier,
        kind: EntityDefnVariant,
        route: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    ) -> Arc<EntityDefn> {
        Arc::new(Self {
            ident,
            subentities: kind.subentities(),
            variant: kind,
            route,
            file,
            range,
        })
    }

    pub fn is_builtin(&self) -> bool {
        match self.variant {
            EntityDefnVariant::Builtin => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
        stmts: Avec<ProcStmt>,
    },
    Type {
        type_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        kind: TypeKind,
        trait_impls: Vec<Arc<EntityDefn>>,
        members: Avec<EntityDefn>,
    },
    EnumVariant {
        ident: CustomIdentifier,
        variant: EnumVariantDefnVariant,
    },
    Builtin,
    TypeField {
        ident: CustomIdentifier,
        ty: EntityRoutePtr,
        field_variant: FieldDefnVariant,
        contract: FieldContract,
    },
    TypeMethod {
        ident: CustomIdentifier,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedEntityRoute,
        this_contract: InputContract,
        method_variant: MethodDefnVariant,
    },
    TraitMethod {
        trai: EntityRoutePtr,
        ident: CustomIdentifier,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedEntityRoute,
        this_contract: InputContract,
        method_variant: MethodDefnVariant,
    },
    TraitMethodImpl {
        trai: EntityRoutePtr,
        ident: CustomIdentifier,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedEntityRoute,
        this_contract: InputContract,
        method_variant: MethodDefnVariant,
    },
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
    entity_route: EntityRoutePtr,
) -> SemanticResultArc<EntityDefn> {
    let source = db.entity_source(entity_route)?;
    match source {
        EntitySource::StaticModuleItem(static_entity_defn) => {
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
                    let signature = try_infer!(db.type_decl(entity_route));
                    (
                        ident,
                        EntityDefnVariant::ty_from_ast(
                            db.upcast(),
                            entity_route,
                            ast_head,
                            not_none!(children),
                            arena,
                            file,
                        )?,
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
                AstKind::TypeMethodDefnHead(ref head) => match entity_route.kind {
                    EntityRouteKind::Root { ident } => todo!(),
                    EntityRouteKind::Package { main, ident } => todo!(),
                    EntityRouteKind::Child { parent: ty, ident } => {
                        let ty_defn = db.entity_defn(ty)?;
                        match ty_defn.variant {
                            EntityDefnVariant::Type {
                                ref type_members, ..
                            } => return Ok(type_members.get(ident).unwrap().clone()),
                            _ => panic!(),
                        }
                    }
                    EntityRouteKind::TraitMember { ty, trai, ident } => todo!(),
                    EntityRouteKind::Input { main } => todo!(),
                    EntityRouteKind::Generic { ident, entity_kind } => todo!(),
                    EntityRouteKind::ThisType => todo!(),
                },
                AstKind::FeatureDecl { ident, ty } => (
                    ident,
                    EntityDefnVariant::feature(db, ty, not_none!(children), arena, file)?,
                ),
                AstKind::MembFeatureDefnHead { ident, ty } => todo!(),
            };
            Ok(EntityDefn::new(
                ident.into(),
                entity_kind,
                entity_route,
                file,
                ast_head.range,
            ))
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Input { .. } => todo!(),
        EntitySource::StaticTypeMember => todo!(),
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
