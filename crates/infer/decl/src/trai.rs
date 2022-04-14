use crate::*;
use atom::symbol_proxy::Symbol;
use fold::LocalStack;
use implement::Implementor;
use map_collect::MapCollect;
use static_decl::StaticTraitMemberDecl;
use vec_dict::HasKey;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDecl {
    pub route: EntityRoutePtr,
    pub members: IdentDict<TraitMemberDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberDecl {
    Method(Arc<MethodDecl>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberImplDecl {
    Method(Arc<MethodDecl>),
}
impl TraitMemberImplDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            TraitMemberImplDecl::Method(method_decl) => {
                TraitMemberImplDecl::Method(method_decl.instantiate(instantiator))
            }
        }
    }
}

impl TraitMemberDecl {
    pub fn from_static(
        db: &dyn DeclQueryGroup,
        static_member_decl: &StaticTraitMemberDecl,
        symbols: &LocalStack<Symbol>,
    ) -> Self {
        match static_member_decl {
            StaticTraitMemberDecl::Method(static_method_decl) => TraitMemberDecl::Method(
                MethodDecl::from_static(db, static_method_decl, None, symbols),
            ),
            StaticTraitMemberDecl::Call => todo!(),
            StaticTraitMemberDecl::Type => todo!(),
        }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            TraitMemberDecl::Method(method_decl) => {
                TraitMemberDecl::Method(method_decl.instantiate(instantiator))
            }
        }
    }

    pub fn implement(
        &self,
        db: &dyn DeclQueryGroup,
        this_ty: EntityRoutePtr,
    ) -> TraitMemberImplDecl {
        let implementor = Implementor {
            db: db.upcast(),
            this_ty,
        };
        match self {
            TraitMemberDecl::Method(method_decl) => {
                TraitMemberImplDecl::Method(method_decl.implement(&implementor))
            }
        }
    }
}

impl HasKey<CustomIdentifier> for TraitMemberDecl {
    fn key(&self) -> CustomIdentifier {
        match self {
            TraitMemberDecl::Method(method_decl) => method_decl.ident,
        }
    }
}

impl TraitDecl {
    pub fn from_static(db: &dyn DeclQueryGroup, trait_decl: &StaticTraitDecl) -> Arc<Self> {
        let symbols = LocalStack::new();
        for generic_placeholder in trait_decl.generic_placeholders.iter() {
            todo!()
        }
        Arc::new(TraitDecl {
            route: db.parse_entity(trait_decl.route, None, &symbols).unwrap(),
            members: trait_decl
                .members
                .iter()
                .map(|member| TraitMemberDecl::from_static(db, member, &symbols))
                .collect(),
        })
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            route: instantiator.instantiate_entity_route(self.route).as_scope(),
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
        EntitySource::Static(builtin_entity_data) => match builtin_entity_data.decl {
            StaticEntityDecl::Func(_) => todo!(),
            StaticEntityDecl::Ty {
                raw_ty_kind,
                visualizer,
            } => todo!(),
            StaticEntityDecl::TyTemplate => todo!(),
            StaticEntityDecl::Trait(ref trait_decl) => Ok(TraitDecl::from_static(db, trait_decl)),
            StaticEntityDecl::Module => todo!(),
        },
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => todo!(),
        EntitySource::Module { file } => todo!(),
        EntitySource::Input { main } => todo!(),
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
