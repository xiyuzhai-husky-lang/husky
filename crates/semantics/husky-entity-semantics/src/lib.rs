mod call_form;
mod dependence;
mod feature;
mod function;
mod module;
mod query;
mod repr;
mod subentities;
mod trai;
mod ty;
mod utils;
mod verify;
mod visual;

pub use call_form::*;
pub use feature::*;
pub use function::*;
pub use query::*;
pub use repr::*;
pub use trai::*;
pub use ty::*;
pub use visual::*;

use avec::Avec;
use fold::{FoldIterItem, FoldableStorage};
use husky_ast::AstVariant;
use husky_atom::{
    context::{AtomContextKind, Symbol},
    AtomContext, AtomContextStandalone,
};
use husky_defn_head::*;
use husky_eager_semantics::*;
use husky_entity_kind::*;
use husky_entity_route::{EntityRoute, EntityRouteVariant};
use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_entity_syntax::EntitySource;
use husky_file::FilePtr;
use husky_lazy_semantics::parse_lazy_stmts;
use husky_lazy_semantics::{LazyExpr, LazyExprVariant, LazyOpnKind, LazyStmt, LazyStmtVariant};
use husky_liason_semantics::*;
use husky_print_utils::{msg_once, p};
use husky_semantics_error::*;
use husky_static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use husky_static_visualizer::StaticVisualTy;
use husky_text::*;
use husky_vm::*;
use husky_word::{
    ContextualIdentifier, CustomIdentifier, IdentDict, Identifier, RootBuiltinIdentifier,
};
use map_collect::MapCollect;
use module::module_defn;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use thin_vec::thin_vec;
use vec_like::VecMapEntry;

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

impl VecMapEntry<CustomIdentifier> for EntityDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident.custom()
    }
}

impl EntityDefn {
    pub fn from_static(
        db: &dyn EntityDefnQueryGroup,
        symbol_context: &mut dyn AtomContext,
        route: EntityRoutePtr,
        static_entity_defn: &'static EntityStaticDefn,
    ) -> Arc<Self> {
        let variant = EntityDefnVariant::from_static(db, symbol_context, static_entity_defn);
        EntityDefn::new(
            db,
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

    pub fn from_generic(
        db: &dyn EntityDefnQueryGroup,
        ident: CustomIdentifier,
        route: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    ) -> Arc<Self> {
        Self::new(db, ident.into(), EntityDefnVariant::Any, route, file, range)
    }

    pub fn this_type(db: &dyn EntityDefnQueryGroup, file: FilePtr, range: TextRange) -> Arc<Self> {
        Self::new(
            db,
            Identifier::Contextual(ContextualIdentifier::ThisType),
            EntityDefnVariant::Any,
            db.intern_entity_route(EntityRoute {
                variant: EntityRouteVariant::ThisType { file, range },
                temporal_arguments: Default::default(),
                spatial_arguments: Default::default(),
            }),
            file,
            range,
        )
    }

    pub(crate) fn new(
        db: &dyn EntityDefnQueryGroup,
        ident: Identifier,
        variant: EntityDefnVariant,
        base_route: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    ) -> Arc<EntityDefn> {
        let entity_defn = Self {
            ident,
            subentities: variant.subentities(),
            variant,
            base_route,
            file,
            range,
        };
        entity_defn.verify(db);
        Arc::new(entity_defn)
    }

    pub fn is_builtin(&self) -> bool {
        match self.variant {
            EntityDefnVariant::Builtin => true,
            _ => false,
        }
    }

    pub fn trait_impl(&self, trai: EntityRoutePtr) -> Option<&Arc<TraitImplDefn>> {
        match self.variant {
            EntityDefnVariant::Ty {
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

    pub fn spatial_parameters(&self) -> &[SpatialParameter] {
        match self.variant {
            EntityDefnVariant::Module { .. } => panic!(),
            EntityDefnVariant::Feature { .. } => panic!(),
            EntityDefnVariant::Function {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Method {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Func {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Proc {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Ty {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Trait {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::EnumVariant { .. } => {
                msg_once!("enum spatial parameters");
                todo!()
            }
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField { .. } => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { .. } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { .. } => todo!(),
            EntityDefnVariant::TargetInput { .. } => todo!(),
            EntityDefnVariant::Any => todo!(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum EntityDefnVariant {
    Module {
        module_items: Avec<EntityDefn>,
        opt_main_defn: Option<Arc<MainDefn>>,
    },
    Feature {
        defn_repr: Arc<DefinitionRepr>,
    },
    Function {
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        output: RangedEntityRoute,
        source: CallFormSource,
    },
    Method {
        spatial_parameters: IdentDict<SpatialParameter>,
        this_modifier: ParameterModifier,
        parameters: Arc<Vec<Parameter>>,
        output_ty: RangedEntityRoute,
        output_modifier: OutputModifier,
        method_defn_kind: MethodDefnKind,
        opt_source: Option<CallFormSource>,
    },
    Func {
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        output: RangedEntityRoute,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    Proc {
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        output: RangedEntityRoute,
        stmts: Avec<ProcStmt>,
    },
    Ty {
        spatial_parameters: IdentDict<SpatialParameter>,
        ty_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        ty_kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDefn>>,
        members: Avec<EntityDefn>,
        opt_type_call: Option<Arc<TypeCallDefn>>,
        visualizer: Arc<Visualizer>,
    },
    Trait {
        spatial_parameters: IdentDict<SpatialParameter>,
        members: IdentDict<Arc<EntityDefn>>,
    },
    EnumVariant {
        enum_variant_defn_variant: EnumVariantDefnVariant,
    },
    Builtin,
    TyField {
        field_ty: EntityRoutePtr,
        field_variant: FieldDefnVariant,
        liason: MemberModifier,
        opt_linkage: Option<__Linkage>,
    },
    TraitAssociatedTypeImpl {
        trai: EntityRoutePtr,
        ty: EntityRoutePtr,
    },
    TraitAssociatedConstSizeImpl {
        value: usize,
    },
    TargetInput,
    Any,
}

impl std::fmt::Debug for EntityDefnVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityDefnVariant::Module { .. } => f.write_str("Module { ... }"),
            EntityDefnVariant::Feature { .. } => f.write_str("Feature { ... }"),
            EntityDefnVariant::Function { .. } => f.write_str("Function { ... }"),
            EntityDefnVariant::Method { .. } => f.write_str("Method { ... }"),
            EntityDefnVariant::Func { .. } => f.write_str("Func { ... }"),
            EntityDefnVariant::Proc { .. } => f.write_str("Proc { ... }"),
            EntityDefnVariant::Ty { .. } => f.write_str("Ty { ... }"),
            EntityDefnVariant::Trait { .. } => f.write_str("Trait { ... }"),
            EntityDefnVariant::EnumVariant { .. } => f.write_str("EnumVariant { ... }"),
            EntityDefnVariant::Builtin => f.write_str("Builtin { ... }"),
            EntityDefnVariant::TyField { .. } => f.write_str("TyField"),
            EntityDefnVariant::TraitAssociatedTypeImpl { .. } => {
                f.write_str("TraitAssociatedTypeImpl { ... }")
            }
            EntityDefnVariant::TraitAssociatedConstSizeImpl { .. } => {
                f.write_str("TraitAssociatedConstSizeImpl { ... }")
            }
            EntityDefnVariant::TargetInput { .. } => f.write_str("Input"),
            EntityDefnVariant::Any => f.write_str("Generic"),
        }
    }
}

impl EntityDefnVariant {
    pub fn from_static(
        db: &dyn EntityDefnQueryGroup,
        symbol_context: &mut dyn AtomContext,
        static_defn: &'static EntityStaticDefn,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::Function {
                spatial_parameters,
                parameters,
                output_ty,
                linkage,
                ..
            } => EntityDefnVariant::Function {
                spatial_parameters: spatial_parameters.map(|spatial_parameter| {
                    SpatialParameter::from_static(
                        symbol_context.entity_syntax_db(),
                        spatial_parameter,
                    )
                }),
                parameters: Arc::new(
                    parameters.map(|parameter| symbol_context.parameter_from_static(parameter)),
                ),
                output: RangedEntityRoute {
                    route: symbol_context.parse_entity_route(output_ty).unwrap(),
                    range: Default::default(),
                },
                source: CallFormSource::Static(linkage),
            },
            EntityStaticDefnVariant::Ty { .. } => {
                EntityDefnVariant::ty_from_static(db, symbol_context, static_defn)
            }
            EntityStaticDefnVariant::Trait {
                base_route,
                spatial_parameters: generic_parameters,
                members,
            } => {
                let mut symbol_context = AtomContextStandalone {
                    db: symbol_context.entity_syntax_db(),
                    opt_this_ty: None,
                    opt_this_contract: None,
                    symbols: (&[] as &[Symbol]).into(),
                    kind: AtomContextKind::Normal,
                    opt_file: Some(db.intern_file(static_defn.dev_src.file.into())),
                };
                let base_route = symbol_context.parse_entity_route(base_route).unwrap();
                let generic_parameters =
                    symbol_context.generic_parameters_from_static(generic_parameters);
                let spatial_arguments =
                    symbol_context.generic_arguments_from_generic_parameters(&generic_parameters);
                let this_trai = symbol_context
                    .db
                    .intern_entity_route(base_route.call(spatial_arguments));
                let member_kinds: Vec<_> = members.map(|member| {
                    (
                        symbol_context.db.intern_word(member.name).custom(),
                        match member.variant {
                            EntityStaticDefnVariant::Method { .. } => {
                                MemberKind::Method { is_lazy: false }
                            }
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
                    spatial_parameters: generic_parameters,
                    members: members.map(|member: &'static EntityStaticDefn| {
                        let route = symbol_context.db.intern_entity_route(EntityRoute::subroute(
                            this_trai,
                            symbol_context.db.intern_word(member.name).custom(),
                            thin_vec![],
                        ));
                        EntityDefn::from_static(db, &mut symbol_context, route, member)
                    }),
                }
            }
            EntityStaticDefnVariant::Module => todo!(),
            EntityStaticDefnVariant::Method {
                this_modifier: this_liason,
                parameters,
                output_ty,
                output_liason,
                spatial_parameters: generic_parameters,
                method_static_defn_kind: method_kind,
                opt_linkage,
            } => EntityDefnVariant::Method {
                spatial_parameters: generic_parameters.map(|static_generic_placeholder| {
                    SpatialParameter::from_static(
                        symbol_context.entity_syntax_db(),
                        static_generic_placeholder,
                    )
                }),
                this_modifier: this_liason,
                parameters: Arc::new(parameters.map(|input_placeholder| {
                    symbol_context.parameter_from_static(input_placeholder)
                })),
                output_ty: RangedEntityRoute {
                    route: symbol_context.parse_entity_route(output_ty).unwrap(),
                    range: Default::default(),
                },
                output_modifier: output_liason,
                method_defn_kind: MethodDefnKind::from_static(symbol_context, method_kind),
                opt_source: opt_linkage.map(|linkage| linkage.into()),
            },
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TyField { .. } => {
                EntityDefnVariant::ty_field_from_static(symbol_context, static_defn)
            }
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
            EntityStaticDefnVariant::EnumVariant => EntityDefnVariant::EnumVariant {
                enum_variant_defn_variant: EnumVariantDefnVariant::Constant,
            },
        }
    }
}

pub(crate) fn main_defn(
    this: &dyn EntityDefnQueryGroup,
    target_entrance: husky_file::FilePtr,
) -> SemanticResultArc<MainDefn> {
    let ast_text = this.ast_text(target_entrance).unwrap();
    for item in ast_text.folded_results.iter() {
        match item.value.as_ref().unwrap().variant {
            AstVariant::MainDefnHead => {
                let ty = RangedEntityRoute {
                    route: this.target_output_ty().unwrap(),
                    range: Default::default(),
                };
                return Ok(Arc::new(MainDefn {
                    defn_repr: DefinitionRepr::LazyBlock {
                        stmts: parse_lazy_stmts(
                            this.upcast(),
                            &ast_text.arena,
                            not_none!(item.opt_children),
                            target_entrance,
                            ty,
                        )?,
                        ty,
                    },
                    file: target_entrance,
                }));
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
    let source = db.entity_source(entity_route).unwrap();
    match source {
        EntitySource::StaticModuleItem(static_defn) => Ok(EntityDefn::from_static(
            db,
            &mut AtomContextStandalone {
                db: db.upcast(),
                opt_this_ty: None,
                opt_this_contract: None,
                symbols: (&[] as &[Symbol]).into(),
                kind: AtomContextKind::Normal,
                opt_file: Some(db.intern_file(static_defn.dev_src.file.into())),
            },
            entity_route,
            static_defn,
        )),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file).unwrap();
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
            let ast = value.as_ref().unwrap();

            let (ident, husky_entity_kind) = match ast.variant {
                AstVariant::TypeDefnHead { ident, .. } => (
                    ident,
                    EntityDefnVariant::ty_from_ast(
                        db,
                        entity_route,
                        ast,
                        not_none!(opt_children),
                        arena,
                        file,
                    )?,
                ),
                AstVariant::CallFormDefnHead {
                    opt_this_liason,
                    ident,
                    ..
                } => match opt_this_liason {
                    Some(_) => return Ok(db.member_defn(entity_route)),
                    None => (
                        ident,
                        EntityDefnVariant::function(db, ast, not_none!(opt_children), arena, file)?,
                    ),
                },
                AstVariant::FieldDefnHead { .. } => return Ok(db.member_defn(entity_route)),
                AstVariant::Use { .. } => todo!(),
                AstVariant::MainDefnHead
                | AstVariant::DatasetConfigDefnHead
                | AstVariant::Stmt(_) => {
                    panic!()
                }
                AstVariant::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => (
                    ident,
                    EntityDefnVariant::enum_variant(db, ident, variant_class, opt_children),
                ),
                AstVariant::FeatureDefnHead {
                    paradigm,
                    ident,
                    return_ty: output_ty,
                } => (
                    ident,
                    EntityDefnVariant::feature(
                        db,
                        entity_route,
                        paradigm,
                        output_ty,
                        opt_children,
                        arena,
                        file,
                    )?,
                ),
                AstVariant::Submodule { ident, source_file } => todo!(),
                AstVariant::Visual => todo!(),
            };
            Ok(EntityDefn::new(
                db,
                ident.ident.into(),
                husky_entity_kind,
                entity_route,
                file,
                ast.range,
            ))
        }
        EntitySource::Module { file } => module_defn(db, entity_route, file),
        EntitySource::TargetInput => {
            msg_once!("use task config for input defn");
            Ok(Arc::new(EntityDefn {
                ident: entity_route.ident(),
                variant: EntityDefnVariant::TargetInput,
                subentities: Default::default(),
                base_route: entity_route,
                file: db.opt_target_entrance().unwrap(),
                range: Default::default(),
            }))
        }
        EntitySource::StaticTypeMember(_) => match entity_route.variant {
            EntityRouteVariant::Child { parent, ident } => {
                let ty_defn = db.entity_defn(parent).unwrap();
                match ty_defn.variant {
                    EntityDefnVariant::Ty { ref ty_members, .. } => Ok(ty_members[ident].clone()),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        },
        EntitySource::StaticTypeAsTraitMember => match entity_route.variant {
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => match trai {
                EntityRoutePtr::Root(RootBuiltinIdentifier::CloneTrait) => {
                    msg_once!("this is a temporary ugly solution");
                    let clone_trait_defn = db
                        .entity_defn(EntityRoutePtr::Root(RootBuiltinIdentifier::CloneTrait))
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
        EntitySource::StaticTraitMember(_) => match entity_route.variant {
            EntityRouteVariant::Child { parent, ident } => {
                let trai_defn = db.entity_defn(parent).unwrap();
                match trai_defn.variant {
                    EntityDefnVariant::Trait { ref members, .. } => Ok(members[ident].clone()),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        },
        EntitySource::Any {
            route,
            ident,
            file,
            range,
        } => Ok(EntityDefn::from_generic(db, ident, route, file, range)),
        EntitySource::StaticEnumVariant(_) => todo!(),
        EntitySource::ThisType { file, range } => Ok(EntityDefn::this_type(db, file, range)),
    }
}

pub(crate) fn subentity_defns(
    this: &dyn EntityDefnQueryGroup,
    scope_id: EntityRoutePtr,
) -> SemanticResultArc<Vec<Arc<EntityDefn>>> {
    let mut defns = Vec::new();
    for defn_result in this
        .subentity_routes(scope_id)
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
