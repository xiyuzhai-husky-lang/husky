use crate::*;
use fold::LocalStack;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDecl {
    pub methods: IdentDict<MethodDecl>,
}

impl TraitDecl {
    pub fn from_static(db: &dyn DeclQueryGroup, trait_decl: &StaticTraitDecl) -> Self {
        let symbols = LocalStack::new();
        for generic_placeholder in trait_decl.generic_placeholders.iter() {
            todo!()
        }
        TraitDecl {
            methods: trait_decl
                .methods
                .iter()
                .map(|method| {
                    MethodDecl::from_static(db, method, db.entity_route_menu().this_type, &symbols)
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
