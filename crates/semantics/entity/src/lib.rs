mod dependence;
mod module;
mod morphism;
mod query;
mod routine;
mod subentities;
mod trai;
mod ty;

use atom::{
    symbol::{Symbol, SymbolContextKind},
    SymbolContext,
};
use entity_route_query::EntitySource;
use map_collect::MapCollect;
use module::module_defn;
pub use morphism::*;
use print_utils::p;
pub use query::*;
pub use routine::*;
pub use trai::*;
pub use ty::*;

use ast::AstKind;
use avec::Avec;
use defn_head::*;
use entity_kind::*;
use entity_route::{EntityRoute, EntityRouteKind};
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use file::FilePtr;
use fold::{FoldIterItem, FoldStorage};
use semantics_eager::*;
use semantics_error::*;
use semantics_lazy::parse_lazy_stmts;
use semantics_lazy::{LazyExpr, LazyExprKind, LazyOpnKind, LazyStmt, LazyStmtKind};
use static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use text::*;
use vec_map::HasKey;
use vm::{FieldContract, InputContract, Linkage, OutputContract};
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
    pub base_route: EntityRoutePtr,
    pub file: FilePtr,
    pub range: TextRange,
}

impl HasKey<CustomIdentifier> for EntityDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident.custom()
    }
}

impl EntityDefn {
    pub fn from_static(
        symbol_context: &SymbolContext,
        route: EntityRoutePtr,
        static_entity_defn: &EntityStaticDefn,
    ) -> Arc<Self> {
        let variant = EntityDefnVariant::from_static(symbol_context, static_entity_defn);
        Self::new(
            symbol_context
                .db
                .intern_word(static_entity_defn.name)
                .ident(),
            variant,
            route,
            symbol_context
                .db
                .intern_file(static_entity_defn.dev_src.file.into()),
            static_entity_defn.dev_src.into(),
        )
    }

    pub(crate) fn new(
        ident: Identifier,
        variant: EntityDefnVariant,
        base_route: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    ) -> Arc<EntityDefn> {
        Arc::new(Self {
            ident,
            subentities: variant.subentities(),
            variant,
            base_route,
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

    pub fn trait_impl(&self, trai: EntityRoutePtr) -> Option<&Arc<TraitImplDefn>> {
        match self.variant {
            EntityDefnVariant::Type {
                ref trait_impls, ..
            } => trait_impls
                .iter()
                .find(|trait_impl| trait_impl.trai == trai),
            _ => panic!(""),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EntityDefnVariant {
    Main(MainDefn),
    Module {
        module_items: Avec<EntityDefn>,
    },
    Feature {
        ty: RangedEntityRoute,
        lazy_stmts: Arc<Vec<Arc<LazyStmt>>>,
    },
    Pattern {},
    Func {
        generic_placeholders: IdentDict<GenericPlaceholder>,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedEntityRoute,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    Proc {
        generic_placeholders: IdentDict<GenericPlaceholder>,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedEntityRoute,
        stmts: Avec<ProcStmt>,
    },
    Type {
        generic_placeholders: IdentDict<GenericPlaceholder>,
        ty_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDefn>>,
        members: Avec<EntityDefn>,
        opt_type_call: Option<Arc<TyCallDefn>>,
    },
    Trait {
        generic_placeholders: IdentDict<GenericPlaceholder>,
        members: IdentDict<Arc<EntityDefn>>,
    },
    EnumVariant {
        ident: RangedCustomIdentifier,
        variant: EnumVariantDefnVariant,
    },
    Builtin,
    TypeField {
        ty: EntityRoutePtr,
        field_variant: FieldDefnVariant,
        contract: FieldContract,
    },
    Method {
        generic_placeholders: IdentDict<GenericPlaceholder>,
        this_contract: InputContract,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output_ty: RangedEntityRoute,
        output_contract: OutputContract,
        method_variant: MethodDefnVariant,
    },
    TraitAssociatedTypeImpl {
        trai: EntityRoutePtr,
        ty: EntityRoutePtr,
    },
    TraitAssociatedConstSizeImpl {
        value: usize,
    },
}

impl EntityDefnVariant {
    pub fn from_static(symbol_context: &SymbolContext, static_defn: &EntityStaticDefn) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::Routine { .. } => todo!(),
            EntityStaticDefnVariant::Type { .. } => {
                Self::ty_from_static(symbol_context, static_defn)
            }
            EntityStaticDefnVariant::Trait {
                base_route,
                generic_placeholders,
                members,
            } => {
                let mut symbol_context = SymbolContext {
                    opt_package_main: symbol_context.opt_package_main,
                    db: symbol_context.db,
                    opt_this_ty: None,
                    opt_this_contract: None,
                    symbols: (&[] as &[Symbol]).into(),
                    kind: SymbolContextKind::Normal,
                };
                let base_route = symbol_context.entity_route_from_str(base_route).unwrap();
                let generic_placeholders =
                    symbol_context.generic_placeholders_from_static(generic_placeholders);
                let generic_arguments = symbol_context
                    .generic_arguments_from_generic_placeholders(&generic_placeholders);
                let this_trai = symbol_context.db.intern_entity_route(EntityRoute {
                    kind: base_route.kind,
                    generic_arguments,
                });
                let member_kinds: Vec<_> = members.map(|member| {
                    (
                        symbol_context.db.intern_word(member.name).custom(),
                        match member.variant {
                            EntityStaticDefnVariant::Method { .. } => MemberKind::Method,
                            EntityStaticDefnVariant::TraitAssociatedType { .. } => {
                                MemberKind::TraitAssociatedType
                            }
                            EntityStaticDefnVariant::TraitAssociatedConstSize => {
                                MemberKind::TraitAssociatedConstSize
                            }
                            _ => panic!(),
                        },
                    )
                });
                symbol_context.kind = SymbolContextKind::Trait {
                    this_trai,
                    member_kinds: &member_kinds,
                };
                EntityDefnVariant::Trait {
                    generic_placeholders,
                    members: members.map(|member| {
                        EntityDefn::from_static(
                            &symbol_context,
                            symbol_context.db.intern_entity_route(EntityRoute::subroute(
                                this_trai,
                                symbol_context.db.intern_word(member.name).custom(),
                                vec![],
                            )),
                            member,
                        )
                    }),
                }
            }
            EntityStaticDefnVariant::Module => todo!(),
            EntityStaticDefnVariant::Method {
                this_contract,
                input_placeholders,
                output_ty,
                output_contract,
                generic_placeholders,
                kind,
            } => EntityDefnVariant::Method {
                generic_placeholders: generic_placeholders.map(|static_generic_placeholder| {
                    GenericPlaceholder::from_static(symbol_context.db, static_generic_placeholder)
                }),
                this_contract,
                input_placeholders: Arc::new(input_placeholders.map(|input_placeholder| {
                    symbol_context.input_placeholder_from_static(input_placeholder)
                })),
                output_ty: RangedEntityRoute {
                    route: symbol_context.entity_route_from_str(output_ty).unwrap(),
                    range: Default::default(),
                },
                output_contract,
                method_variant: MethodDefnVariant::from_static(symbol_context, kind),
            },
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TypeField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
        }
    }
}

pub(crate) fn main_defn(
    this: &dyn EntityDefnQueryGroup,
    main_file: file::FilePtr,
) -> SemanticResultArc<MainDefn> {
    let ast_text = this.ast_text(main_file)?;
    for item in ast_text.folded_results.iter() {
        match item.value.as_ref()?.kind {
            AstKind::MainDefn => {
                return Ok(Arc::new(MainDefn {
                    stmts: parse_lazy_stmts(
                        &[],
                        this.upcast(),
                        &ast_text.arena,
                        not_none!(item.opt_children),
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
        EntitySource::StaticModuleItem(static_defn) => Ok(EntityDefn::from_static(
            &SymbolContext {
                opt_package_main: None,
                db: db.upcast(),
                opt_this_ty: None,
                opt_this_contract: None,
                symbols: (&[] as &[Symbol]).into(),
                kind: SymbolContextKind::Normal,
            },
            entity_route,
            static_defn,
        )),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let arena = &ast_text.arena;
            let FoldIterItem {
                value,
                opt_children: children,
                ..
            } = ast_text
                .folded_results
                .iter_from(token_group_index)
                .next()
                .unwrap();
            let ast_head = value.as_ref()?;

            let (ident, entity_kind) = match ast_head.kind {
                AstKind::TypeDefnHead {
                    ident,
                    kind,
                    generic_placeholders: ref generics,
                } => {
                    let signature = derived_unwrap!(db.ty_decl(entity_route));
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
                AstKind::Use { .. } => todo!(),
                AstKind::MainDefn | AstKind::DatasetConfigDefnHead | AstKind::Stmt(_) => panic!(),
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => (
                    ident,
                    EntityDefnVariant::enum_variant(db, ident, variant_class, children),
                ),
                AstKind::FieldDefnHead { .. } | AstKind::TypeMethodDefnHead(_) => {
                    return Ok(db.member_defn(entity_route))
                }
                AstKind::FeatureDecl { ident, ty } => (
                    ident,
                    EntityDefnVariant::feature(db, ty, not_none!(children), arena, file)?,
                ),
                AstKind::Submodule { ident, source_file } => todo!(),
            };
            Ok(EntityDefn::new(
                ident.ident.into(),
                entity_kind,
                entity_route,
                file,
                ast_head.range,
            ))
        }
        EntitySource::Module { file } => module_defn(db, entity_route, file),
        EntitySource::Input { .. } => todo!(),
        EntitySource::StaticTypeMember => match entity_route.kind {
            EntityRouteKind::Child { parent: ty, ident } => {
                let ty_defn = db.entity_defn(ty).unwrap();
                match ty_defn.variant {
                    EntityDefnVariant::Type {
                        ty_members: ref type_members,
                        ..
                    } => Ok(type_members[ident].clone()),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        },
        EntitySource::StaticTypeAsTraitMember => match entity_route.kind {
            EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
                let ty_defn = db.entity_defn(ty)?;
                let trai_impl_defn = ty_defn
                    .trait_impl(trai)
                    .expect("todo: trait_impl not found");
                Ok(trai_impl_defn.member_impl(ident).clone())
            }
            _ => panic!(),
        },
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
