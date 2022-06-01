mod dependence;
mod feature;
mod function;
mod module;
mod query;
mod subentities;
mod trai;
mod ty;

use atom::{
    context::{AtomContextKind, Symbol},
    AtomContext, AtomContextStandalone,
};
use entity_syntax::EntityLocus;
pub use feature::*;
pub use function::*;
use map_collect::MapCollect;
use module::module_defn;
use print_utils::{msg_once, p};
pub use query::*;
pub use trai::*;
pub use ty::*;

use ast::AstVariant;
use avec::Avec;
use defn_head::*;
use entity_kind::*;
use entity_route::{EntityRoute, EntityRouteKind};
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use file::FilePtr;
use fold::{FoldIterItem, FoldableStorage};
use semantics_eager::*;
use semantics_error::*;
use semantics_lazy::parse_lazy_stmts;
use semantics_lazy::{LazyExpr, LazyExprVariant, LazyOpnKind, LazyStmt, LazyStmtVariant};
use static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use text::*;
use vec_map::HasKey;
use visual_semantics::VisualizerSource;
use vm::{InputLiason, Linkage, MemberLiason, OutputLiason};
use word::{CustomIdentifier, IdentDict, Identifier, RootIdentifier};

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
        symbol_context: &mut dyn AtomContext,
        route: EntityRoutePtr,
        static_entity_defn: &EntityStaticDefn,
    ) -> Arc<Self> {
        let variant = EntityDefnVariant::from_static(symbol_context, static_entity_defn);
        Self::new(
            symbol_context
                .entity_syntax_db()
                .intern_word(static_entity_defn.name)
                .ident(),
            variant,
            route,
            symbol_context
                .entity_syntax_db()
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

    pub fn trait_member_defn(&self, ident: CustomIdentifier) -> Option<&Arc<EntityDefn>> {
        match self.variant {
            EntityDefnVariant::Trait { ref members, .. } => members.get_entry(ident),
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
        defn_repr: DefinitionRepr,
    },
    Func {
        generic_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<InputParameter>>,
        output: RangedEntityRoute,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    Proc {
        generic_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<InputParameter>>,
        output: RangedEntityRoute,
        stmts: Avec<ProcStmt>,
    },
    Type {
        generic_parameters: IdentDict<SpatialParameter>,
        ty_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDefn>>,
        members: Avec<EntityDefn>,
        opt_type_call: Option<Arc<TyCallDefn>>,
        opt_visualizer_source: Option<VisualizerSource>,
    },
    Trait {
        generic_parameters: IdentDict<SpatialParameter>,
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
        liason: MemberLiason,
    },
    Method {
        generic_parameters: IdentDict<SpatialParameter>,
        this_contract: InputLiason,
        parameters: Arc<Vec<InputParameter>>,
        output_ty: RangedEntityRoute,
        output_liason: OutputLiason,
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
    pub fn from_static(
        symbol_context: &mut dyn AtomContext,
        static_defn: &EntityStaticDefn,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::Routine { .. } => todo!(),
            EntityStaticDefnVariant::Ty { .. } => Self::ty_from_static(symbol_context, static_defn),
            EntityStaticDefnVariant::Trait {
                base_route,
                generic_parameters,
                members,
            } => {
                let mut symbol_context = AtomContextStandalone {
                    opt_package_main: symbol_context.opt_package_main(),
                    db: symbol_context.entity_syntax_db(),
                    opt_this_ty: None,
                    opt_this_contract: None,
                    symbols: (&[] as &[Symbol]).into(),
                    kind: AtomContextKind::Normal,
                };
                let base_route = symbol_context.entity_route_from_str(base_route).unwrap();
                let generic_parameters =
                    symbol_context.generic_parameters_from_static(generic_parameters);
                let generic_arguments =
                    symbol_context.generic_arguments_from_generic_parameters(&generic_parameters);
                let this_trai = symbol_context.db.intern_entity_route(EntityRoute {
                    kind: base_route.kind,
                    spatial_arguments: generic_arguments,
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
                symbol_context.kind = AtomContextKind::Trait {
                    this_trai,
                    member_kinds: &member_kinds,
                };
                EntityDefnVariant::Trait {
                    generic_parameters,
                    members: members.map(|member| {
                        let route = symbol_context.db.intern_entity_route(EntityRoute::subroute(
                            this_trai,
                            symbol_context.db.intern_word(member.name).custom(),
                            vec![],
                        ));
                        EntityDefn::from_static(&mut symbol_context, route, member)
                    }),
                }
            }
            EntityStaticDefnVariant::Module => todo!(),
            EntityStaticDefnVariant::Method {
                this_contract,
                input_parameters: parameters,
                output_ty,
                output_liason,
                generic_parameters: generic_parameters,
                ref kind,
            } => EntityDefnVariant::Method {
                generic_parameters: generic_parameters.map(|static_generic_placeholder| {
                    SpatialParameter::from_static(
                        symbol_context.entity_syntax_db(),
                        static_generic_placeholder,
                    )
                }),
                this_contract,
                parameters: Arc::new(parameters.map(|input_placeholder| {
                    symbol_context.input_placeholder_from_static(input_placeholder)
                })),
                output_ty: RangedEntityRoute {
                    route: symbol_context.entity_route_from_str(output_ty).unwrap(),
                    range: Default::default(),
                },
                output_liason,
                method_variant: MethodDefnVariant::from_static(symbol_context, kind),
            },
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TyField { .. } => todo!(),
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
        match item.value.as_ref()?.variant {
            AstVariant::MainDefn => {
                return Ok(Arc::new(MainDefn {
                    defn_repr: DefinitionRepr::LazyBlock {
                        stmts: parse_lazy_stmts(
                            &[],
                            this.upcast(),
                            &ast_text.arena,
                            not_none!(item.opt_children),
                            main_file,
                        )?,
                    },
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
    let source = db.entity_locus(entity_route)?;
    match source {
        EntityLocus::StaticModuleItem(static_defn) => Ok(EntityDefn::from_static(
            &mut AtomContextStandalone {
                opt_package_main: None,
                db: db.upcast(),
                opt_this_ty: None,
                opt_this_contract: None,
                symbols: (&[] as &[Symbol]).into(),
                kind: AtomContextKind::Normal,
            },
            entity_route,
            static_defn,
        )),
        EntityLocus::WithinBuiltinModule => todo!(),
        EntityLocus::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let arena = &ast_text.arena;
            let FoldIterItem {
                value,
                opt_children,
                ..
            } = ast_text
                .folded_results
                .iter_from(token_group_index)
                .next()
                .unwrap();
            let ast_head = value.as_ref()?;

            let (ident, entity_kind) = match ast_head.variant {
                AstVariant::TypeDefnHead {
                    ident,
                    kind,
                    generic_parameters: ref generics,
                } => {
                    let signature = derived_unwrap!(db.ty_decl(entity_route));
                    (
                        ident,
                        EntityDefnVariant::ty_from_ast(
                            db.upcast(),
                            entity_route,
                            ast_head,
                            not_none!(opt_children),
                            arena,
                            file,
                        )?,
                    )
                }
                AstVariant::CallFormDefnHead(ref head) => match head.opt_this_contract {
                    Some(_) => return Ok(db.member_defn(entity_route)),
                    None => (
                        head.ident,
                        EntityDefnVariant::function(
                            db.upcast(),
                            head,
                            not_none!(opt_children),
                            arena,
                            file,
                        )?,
                    ),
                },
                AstVariant::FieldDefnHead { .. } => return Ok(db.member_defn(entity_route)),
                AstVariant::PatternDefnHead => todo!(),
                AstVariant::Use { .. } => todo!(),
                AstVariant::MainDefn | AstVariant::DatasetConfigDefnHead | AstVariant::Stmt(_) => {
                    panic!()
                }
                AstVariant::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => (
                    ident,
                    EntityDefnVariant::enum_variant(db, ident, variant_class, opt_children),
                ),
                AstVariant::FeatureDecl { ident, ty } => (
                    ident,
                    EntityDefnVariant::feature(db, ty, not_none!(opt_children), arena, file)?,
                ),
                AstVariant::Submodule { ident, source_file } => todo!(),
                AstVariant::Visual => todo!(),
            };
            Ok(EntityDefn::new(
                ident.ident.into(),
                entity_kind,
                entity_route,
                file,
                ast_head.range,
            ))
        }
        EntityLocus::Module { file } => module_defn(db, entity_route, file),
        EntityLocus::Input { .. } => todo!(),
        EntityLocus::StaticTypeMember => match entity_route.kind {
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
        EntityLocus::StaticTypeAsTraitMember => match entity_route.kind {
            EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => match trai {
                EntityRoutePtr::Root(RootIdentifier::CloneTrait) => {
                    msg_once!("this is a temporary ugly solution");
                    let clone_trait_defn = db
                        .entity_defn(EntityRoutePtr::Root(RootIdentifier::CloneTrait))
                        .unwrap();
                    Ok(clone_trait_defn.trait_member_defn(ident).unwrap().clone())
                }
                _ => {
                    let ty_defn = db.entity_defn(ty)?;
                    p!(ty, trai);
                    let trai_impl_defn = ty_defn
                        .trait_impl(trai)
                        .expect("todo: trait_impl not found");
                    Ok(trai_impl_defn.member_impl(ident).clone())
                }
            },
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
