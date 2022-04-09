use crate::*;
use static_decl::StaticMethodDecl;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TraitDecl {
    pub methods: IdentDict<MethodDecl>,
}

impl TraitDecl {
    fn from_static(db: &dyn DeclQueryGroup, trait_decl: &StaticTraitDecl) -> Self {
        TraitDecl {
            methods: trait_decl
                .methods
                .iter()
                .map(|method| {
                    MethodDecl::from_static(
                        db,
                        db.entity_route_menu().this_type,
                        trait_decl.generic_placeholders,
                        method,
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
