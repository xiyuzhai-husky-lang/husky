use entity_route_query::EntityLocus;
use static_defn::EntityStaticDefnVariant;

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
    let scope_source = db.compile_time().entity_locus(ty).unwrap();
    match scope_source {
        EntityLocus::StaticModuleItem(static_defn) => match static_defn.variant {
            EntityStaticDefnVariant::Routine { .. } => todo!(),
            EntityStaticDefnVariant::Type { visualizer, .. } => Arc::new(visualizer.into()),
            EntityStaticDefnVariant::Module => todo!(),
            EntityStaticDefnVariant::Trait { .. } => todo!(),
            EntityStaticDefnVariant::Method {
                this_contract,
                input_parameters: inputs,
                output_ty,
                output_contract,
                generic_parameters: generic_placeholders,
                ref kind,
            } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TypeField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
        },
        EntityLocus::WithinBuiltinModule => todo!(),
        EntityLocus::WithinModule {
            file,
            token_group_index,
        } => todo!(),
        EntityLocus::Module { file } => todo!(),
        EntityLocus::Input { main } => todo!(),
        EntityLocus::StaticTypeMember => todo!(),
        EntityLocus::StaticTypeAsTraitMember => todo!(),
    }
}
