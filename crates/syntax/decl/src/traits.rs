use crate::*;
use word::IdentMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TraitDecl {
    pub members: IdentMap<MembDecl>,
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
            StaticEntityDecl::Trait { members } => Ok(Arc::new(TraitDecl {
                members: members
                    .iter()
                    .map(|member| {
                        (
                            db.intern_word(member.name).custom(),
                            MembDecl::from_static(db, &member.variant),
                        )
                    })
                    .collect(),
            })),
            StaticEntityDecl::Module => todo!(),
        },
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => todo!(),
        EntitySource::Module { file } => todo!(),
        EntitySource::Contextual { main, ident } => todo!(),
    }
}
