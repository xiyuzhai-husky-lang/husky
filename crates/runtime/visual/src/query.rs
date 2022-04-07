use crate::*;
#[salsa::query_group(VisualQueryGroupStorage)]
pub trait VisualQueryGroup: AskCompileTime {
    fn visualizer(&self, version: usize, ty: EntityRoutePtr) -> Arc<RuntimeVisualizer>;
}

fn visualizer(
    db: &dyn VisualQueryGroup,
    version: usize,
    ty: EntityRoutePtr,
) -> Arc<RuntimeVisualizer> {
    let scope_source = db.compile_time(version).entity_source(ty).unwrap();
    match scope_source {
        EntitySource::Builtin(builtin_entity_data) => match builtin_entity_data.decl {
            StaticEntityDecl::Func(_) => todo!(),
            StaticEntityDecl::Ty { ref visualizer, .. } => Arc::new(visualizer.into()),
            StaticEntityDecl::Module => todo!(),
            StaticEntityDecl::TyTemplate => todo!(),
            StaticEntityDecl::Trait { .. } => todo!(),
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
