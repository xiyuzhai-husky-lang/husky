use crate::*;
use atom::{
    symbol::{Symbol, SymbolContextKind},
    SymbolContext,
};
use fold::LocalStack;
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::p;
use static_defn::StaticTraitMemberDecl;
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
        static_member_decl: &StaticTraitMemberDecl,
        symbol_context: &SymbolContext,
    ) -> Self {
        match static_member_decl {
            StaticTraitMemberDecl::Method(static_method_decl) => TraitMemberDecl::Method(
                MethodDecl::from_static(db, static_method_decl, symbol_context),
            ),
            StaticTraitMemberDecl::Call => todo!(),
            StaticTraitMemberDecl::Type { name, traits } => TraitMemberDecl::Type {
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
    pub fn from_static(db: &dyn DeclQueryGroup, trait_decl: &StaticTraitDecl) -> Arc<Self> {
        let generic_placeholders =
            db.parse_generic_placeholders_from_static(trait_decl.generic_placeholders);
        let symbols = db.symbols_from_generic_placeholders(&generic_placeholders);
        let members: Vec<_> = trait_decl
            .members
            .map(|member| (db.intern_word(member.name()).custom(), member.kind()));
        let symbol_context_kind = SymbolContextKind::Trait { members: &members };
        let symbol_context = SymbolContext {
            opt_package_main: None,
            db: db.upcast(),
            opt_this_ty: None,
            symbols: &symbols,
            kind: symbol_context_kind,
        };
        Arc::new(TraitDecl {
            base_route: symbol_context
                .entity_route_from_str(trait_decl.base_route)
                .unwrap(),
            members: trait_decl
                .members
                .iter()
                .map(|member| TraitMemberDecl::from_static(db, member, &symbol_context))
                .collect(),
        })
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
        EntitySource::StaticModuleItem(builtin_entity_data) => match builtin_entity_data.variant {
            StaticEntityDefnVariant::Func(_) => todo!(),
            StaticEntityDefnVariant::Type(_) => todo!(),
            StaticEntityDefnVariant::Trait(ref trait_decl) => {
                Ok(TraitDecl::from_static(db, trait_decl))
            }
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
        clone_trait: TraitDecl::from_static(db, &CLONE_TRAIT_DECL),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDeclMenu {
    pub clone_trait: Arc<TraitDecl>,
}
