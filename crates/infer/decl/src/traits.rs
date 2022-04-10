use crate::*;
use atom::symbol_proxy::Symbol;
use fold::LocalStack;
use vec_dict::HasKey;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDecl {
    pub members: IdentDict<TraitMemberDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberDecl {
    Method(Arc<MethodDecl>),
}

impl TraitMemberDecl {
    pub fn from_static(
        db: &dyn DeclQueryGroup,
        static_member_decl: &StaticTraitMemberDecl,
        this_ty: EntityRoutePtr,
        symbols: &LocalStack<Symbol>,
    ) -> Self {
        match static_member_decl {
            StaticTraitMemberDecl::Method(static_method_decl) => TraitMemberDecl::Method(
                MethodDecl::from_static(db, static_method_decl, this_ty, symbols),
            ),
            StaticTraitMemberDecl::Call => todo!(),
            StaticTraitMemberDecl::Type => todo!(),
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
    pub fn from_static(db: &dyn DeclQueryGroup, trait_decl: &StaticTraitDecl) -> Self {
        let symbols = LocalStack::new();
        for generic_placeholder in trait_decl.generic_placeholders.iter() {
            todo!()
        }
        TraitDecl {
            members: trait_decl
                .members
                .iter()
                .map(|member| {
                    TraitMemberDecl::from_static(
                        db,
                        member,
                        db.entity_route_menu().this_type,
                        &symbols,
                    )
                })
                .collect(),
        }
    }
}

pub(crate) fn trait_decl(
    db: &dyn DeclQueryGroup,
    entity_route: EntityRoutePtr,
) -> InferResultArc<TraitDecl> {
    let entity_source = db.entity_source(entity_route).unwrap();
    match entity_source {
        EntitySource::Builtin(builtin_entity_data) => match builtin_entity_data.decl {
            StaticEntityDecl::Func(_) => todo!(),
            StaticEntityDecl::Ty {
                raw_ty_kind,
                visualizer,
            } => todo!(),
            StaticEntityDecl::TyTemplate => todo!(),
            StaticEntityDecl::Trait(ref trait_decl) => {
                Ok(Arc::new(TraitDecl::from_static(db, trait_decl)))
            }
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
