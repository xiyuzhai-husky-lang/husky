use crate::*;
use atom::{
    symbol::{Symbol, SymbolContextKind},
    SymbolContext,
};
use fold::LocalStack;
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::p;
use static_defn::TraitMemberStaticDefn;
use vec_dict::HasKey;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDecl {
    pub base_route: EntityRoutePtr,
    pub members: IdentDict<TraitMemberDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberDecl {
    Method(Arc<MethodDecl>),
    Type {
        ident: CustomIdentifier,
        traits: Vec<EntityRoutePtr>,
    },
    ConstSize(usize),
    Call {},
}

impl TraitMemberDecl {
    pub fn from_static(
        db: &dyn DeclQueryGroup,
        static_member_decl: &TraitMemberStaticDefn,
        symbol_context: &SymbolContext,
    ) -> Self {
        match static_member_decl {
            TraitMemberStaticDefn::Method(static_method_decl) => TraitMemberDecl::Method(
                MethodDecl::from_static(db, static_method_decl, symbol_context),
            ),
            TraitMemberStaticDefn::Call => todo!(),
            TraitMemberStaticDefn::Type { name, traits } => TraitMemberDecl::Type {
                ident: db.intern_word(name).custom(),
                traits: traits.map(|trai| symbol_context.entity_route_from_str(trai).unwrap()),
            },
        }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            TraitMemberDecl::Method(method_decl) => {
                TraitMemberDecl::Method(method_decl.instantiate(instantiator))
            }
            TraitMemberDecl::Type { ident, traits } => TraitMemberDecl::Type {
                ident: *ident,
                traits: traits.map(|trai| instantiator.instantiate_entity_route(*trai).as_scope()),
            },
            TraitMemberDecl::ConstSize(_) => todo!(),
            TraitMemberDecl::Call {} => todo!(),
        }
    }

    pub fn implement(
        &self,
        db: &dyn DeclQueryGroup,
        implementor: &Implementor,
        // member_impls: &[(CustomIdentifier, GenericArgument)],
    ) -> TraitMemberImplDecl {
        // let implementor = Implementor::new(db.upcast(), this_ty, member_impls);
        match self {
            TraitMemberDecl::Method(method_decl) => {
                TraitMemberImplDecl::Method(method_decl.implement(&implementor))
            }
            TraitMemberDecl::Type { ident, traits } => {
                if traits.len() > 0 {
                    todo!("verify traits are satisfied")
                }
                let ty = implementor.generic_argument(*ident).as_scope();
                TraitMemberImplDecl::AssociatedType { ident: *ident, ty }
            }
            TraitMemberDecl::ConstSize(_) => todo!(),
            TraitMemberDecl::Call {} => todo!(),
        }
    }
}

impl HasKey<CustomIdentifier> for TraitMemberDecl {
    fn key(&self) -> CustomIdentifier {
        match self {
            TraitMemberDecl::Method(method_decl) => method_decl.ident,
            TraitMemberDecl::Type { ident, .. } => *ident,
            TraitMemberDecl::ConstSize(_) => todo!(),
            TraitMemberDecl::Call {} => todo!(),
        }
    }
}

impl TraitDecl {
    pub fn from_static(db: &dyn DeclQueryGroup, static_defn: &EntityStaticDefn) -> Arc<Self> {
        match static_defn.variant {
            StaticEntityDefnVariant::Trait {
                base_route,
                ref generic_placeholders,
                ref members,
            } => {
                let generic_placeholders =
                    db.parse_generic_placeholders_from_static(generic_placeholders);
                let symbols = db.symbols_from_generic_placeholders(&generic_placeholders);
                let member_context: Vec<_> =
                    members.map(|member| (db.intern_word(member.name()).custom(), member.kind()));
                let symbol_context_kind = SymbolContextKind::Trait {
                    members: &member_context,
                };
                let symbol_context = SymbolContext {
                    opt_package_main: None,
                    db: db.upcast(),
                    opt_this_ty: None,
                    symbols: &symbols,
                    kind: symbol_context_kind,
                };
                Arc::new(TraitDecl {
                    base_route: symbol_context.entity_route_from_str(base_route).unwrap(),
                    members: members
                        .iter()
                        .map(|member| TraitMemberDecl::from_static(db, member, &symbol_context))
                        .collect(),
                })
            }
            _ => panic!(),
        }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            base_route: instantiator
                .instantiate_entity_route(self.base_route)
                .as_scope(),
            members: self.members.map(|member| member.instantiate(instantiator)),
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
            StaticEntityDefnVariant::Func(_) => todo!(),
            StaticEntityDefnVariant::Type { .. } => todo!(),
            StaticEntityDefnVariant::Trait { .. } => Ok(TraitDecl::from_static(db, static_defn)),
            StaticEntityDefnVariant::Module => todo!(),
        },
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => todo!(),
        EntitySource::Module { file } => todo!(),
        EntitySource::Input { main } => todo!(),
        EntitySource::StaticTypeMember => todo!(),
    }
}

pub(crate) fn trait_decl_menu(db: &dyn DeclQueryGroup) -> Arc<TraitDeclMenu> {
    Arc::new(TraitDeclMenu {
        clone_trait: TraitDecl::from_static(db, &CLONE_TRAIT_DEFN),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDeclMenu {
    pub clone_trait: Arc<TraitDecl>,
}
