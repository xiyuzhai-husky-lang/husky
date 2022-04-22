use static_defn::StaticEntityDefnVariant;

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
        EntitySource::StaticModuleItem(builtin_entity_data) => match builtin_entity_data.variant {
            StaticEntityDefnVariant::Func(_) => todo!(),
            StaticEntityDefnVariant::Type { visualizer, .. } => Arc::new(visualizer.into()),
            StaticEntityDefnVariant::Module => todo!(),
            StaticEntityDefnVariant::Trait { .. } => todo!(),
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
