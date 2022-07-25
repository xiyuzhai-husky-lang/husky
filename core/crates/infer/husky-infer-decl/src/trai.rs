use crate::*;
use entity_kind::MemberKind;
use husky_atom::{
    context::{AtomContextKind, Symbol},
    AtomContext, AtomContextStandalone,
};
use husky_check_utils::should_eq;
use husky_implement::{Implementable, ImplementationContext};
use husky_instantiate::{Instantiable, InstantiationContext};
use husky_word::IdentDict;
use map_collect::MapCollect;
use thin_vec::{thin_vec, ThinVec};
use vec_like::VecMapEntry;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDecl {
    pub trai: EntityRoutePtr,
    pub generic_parameters: IdentDict<SpatialParameter>,
    pub members: IdentDict<TraitMemberDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberDecl {
    Method(Arc<CallFormDecl>),
    Type {
        ident: CustomIdentifier,
        traits: Vec<EntityRoutePtr>,
    },
    ConstSize(usize),
    Call {},
}

impl TraitMemberDecl {
    pub fn from_static(
        symbol_context: &mut dyn AtomContext,
        route: EntityRoutePtr,
        static_member_defn: &EntityStaticDefn,
    ) -> Self {
        match static_member_defn.variant {
            EntityStaticDefnVariant::Method { .. } => TraitMemberDecl::Method(
                CallFormDecl::from_static(route, symbol_context, static_member_defn),
            ),
            EntityStaticDefnVariant::TraitAssociatedType { trai, traits } => {
                TraitMemberDecl::Type {
                    ident: symbol_context
                        .entity_syntax_db()
                        .intern_word(static_member_defn.name)
                        .custom(),
                    traits: traits.map(|trai| symbol_context.parse_entity_route(trai).unwrap()),
                }
            }
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            _ => panic!(),
        }
    }

    pub fn implement(
        &self,
        db: &dyn DeclQueryGroup,
        implementor: &ImplementationContext,
    ) -> TraitMemberImplDecl {
        match self {
            TraitMemberDecl::Method(call_form_decl) => {
                TraitMemberImplDecl::Method(call_form_decl.implement(&implementor))
            }
            TraitMemberDecl::Type { ident, traits } => {
                if traits.len() > 0 {
                    todo!("verify traits are satisfied")
                }
                let ty = implementor.spatial_argument(*ident).take_entity_route();
                TraitMemberImplDecl::AssociatedType { ident: *ident, ty }
            }
            TraitMemberDecl::ConstSize(_) => todo!(),
            TraitMemberDecl::Call {} => todo!(),
        }
    }
}

impl Instantiable for TraitMemberDecl {
    type Target = Self;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self {
        match self {
            TraitMemberDecl::Method(call_form_decl) => {
                TraitMemberDecl::Method(call_form_decl.instantiate(ctx))
            }
            TraitMemberDecl::Type { ident, traits } => TraitMemberDecl::Type {
                ident: *ident,
                traits: traits.map(|trai| trai.instantiate(ctx).take_entity_route()),
            },
            TraitMemberDecl::ConstSize(_) => todo!(),
            TraitMemberDecl::Call {} => todo!(),
        }
    }
}

impl VecMapEntry<CustomIdentifier> for TraitMemberDecl {
    fn key(&self) -> CustomIdentifier {
        match self {
            TraitMemberDecl::Method(call_form_decl) => call_form_decl.ident(),
            TraitMemberDecl::Type { ident, .. } => *ident,
            TraitMemberDecl::ConstSize(_) => todo!(),
            TraitMemberDecl::Call {} => todo!(),
        }
    }
}

impl TraitDecl {
    pub fn from_static(db: &dyn DeclQueryGroup, static_defn: &EntityStaticDefn) -> Arc<Self> {
        match static_defn.variant {
            EntityStaticDefnVariant::Trait {
                base_route,
                spatial_parameters: ref generic_parameters,
                ref members,
            } => {
                let generic_parameters = db.generic_parameters_from_static(generic_parameters);
                let symbols = db.symbols_from_generic_parameters(&generic_parameters);
                let member_context: Vec<_> = members.map(|member| {
                    (
                        db.intern_word(member.name).custom(),
                        match member.variant {
                            EntityStaticDefnVariant::Method { .. } => {
                                MemberKind::Method { is_lazy: false }
                            }
                            EntityStaticDefnVariant::TraitAssociatedType { .. } => {
                                MemberKind::TraitAssociatedType
                            }
                            _ => panic!(),
                        },
                    )
                });
                let mut symbol_context = AtomContextStandalone {
                    opt_package_main: None,
                    db: db.upcast(),
                    opt_this_ty: None,
                    opt_this_contract: None,
                    symbols: symbols.into(),
                    kind: AtomContextKind::Normal,
                };
                let base_route = symbol_context.parse_entity_route(base_route).unwrap();
                let generic_arguments =
                    db.generic_arguments_from_generic_parameters(&generic_parameters);
                should_eq!(base_route.spatial_arguments.len(), 0);
                let trai = db.intern_entity_route(EntityRoute {
                    kind: base_route.kind,
                    temporal_arguments: thin_vec![],
                    spatial_arguments: generic_arguments,
                });
                symbol_context.kind = AtomContextKind::Trait {
                    this_trai: trai,
                    member_kinds: &member_context,
                };
                Arc::new(TraitDecl {
                    trai,
                    generic_parameters,
                    members: members
                        .iter()
                        .map(|member| {
                            TraitMemberDecl::from_static(
                                &mut symbol_context,
                                db.ty_as_trai_subroute(
                                    EntityRoutePtr::ThisType,
                                    trai,
                                    db.intern_word(member.name).custom(),
                                    thin_vec![],
                                ),
                                member,
                            )
                        })
                        .collect(),
                })
            }
            _ => panic!(),
        }
    }

    pub fn instantiate(
        &self,
        db: &dyn DeclQueryGroup,
        dst_generics: &[SpatialArgument],
    ) -> Arc<Self> {
        should_eq!(self.generic_parameters.len(), dst_generics.len());
        let ctx = InstantiationContext {
            db: db.upcast(),
            spatial_parameters: &self.generic_parameters,
            spatial_arguments: dst_generics,
        };
        Arc::new(TraitDecl {
            trai: self.trai.instantiate(&ctx).take_entity_route(),
            generic_parameters: Default::default(),
            members: self
                .members
                .iter()
                .map(|member| member.instantiate(&ctx))
                .collect(),
        })
    }
}

pub(crate) fn trait_decl(
    db: &dyn DeclQueryGroup,
    entity_route: EntityRoutePtr,
) -> InferResultArc<TraitDecl> {
    let entity_source = db.entity_source(entity_route).unwrap();
    match entity_source {
        EntitySource::StaticModuleItem(static_defn) => match static_defn.variant {
            EntityStaticDefnVariant::Function { .. } => todo!(),
            EntityStaticDefnVariant::Ty { .. } => todo!(),
            EntityStaticDefnVariant::Trait { .. } => {
                let base_decl = TraitDecl::from_static(db, static_defn);
                if entity_route.spatial_arguments.len() > 0 {
                    Ok(base_decl.instantiate(db, &entity_route.spatial_arguments))
                } else {
                    Ok(base_decl)
                }
            }
            EntityStaticDefnVariant::Module => todo!(),
            EntityStaticDefnVariant::Method { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TyField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty: route } => todo!(),
        },
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => todo!(),
        EntitySource::Module { file } => todo!(),
        EntitySource::Input { main } => todo!(),
        EntitySource::StaticTypeMember(_) => todo!(),
        EntitySource::StaticTraitMember(_) => todo!(),
        EntitySource::StaticTypeAsTraitMember => todo!(),
    }
}

// pub(crate) fn trait_decl_menu(db: &dyn DeclQueryGroup) -> Arc<TraitDeclMenu> {
//     Arc::new(TraitDeclMenu {
//         clone_trait: TraitDecl::from_static(db, &CLONE_TRAIT_DEFN),
//     })
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct TraitDeclMenu {
//     pub clone_trait: Arc<TraitDecl>,
// }
