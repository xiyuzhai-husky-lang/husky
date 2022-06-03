use crate::*;
use entity_syntax::EntityLocus;
use instruction_gen::new_visual_instruction_sheet;
use print_utils::p;
use semantics_entity::EntityDefnVariant;
use static_defn::EntityStaticDefnVariant;
use visual_semantics::VisualizerSource;
use visual_syntax::TRIVIAL_VISUALIZER;

#[salsa::query_group(VisualQueryGroupStorage)]
pub trait VisualQueryGroup: AskCompileTime {
    fn visualizer(&self, ty: EntityRoutePtr) -> Arc<RuntimeVisualizer>;
}

fn visualizer(db: &dyn VisualQueryGroup, ty: EntityRoutePtr) -> Arc<RuntimeVisualizer> {
    let ty_defn = db.compile_time().entity_defn(ty).unwrap();
    Arc::new(match ty_defn.variant {
        EntityDefnVariant::Ty {
            ref opt_visualizer_source,
            ..
        } => match opt_visualizer_source {
            Some(visualizer_source) => match visualizer_source {
                VisualizerSource::Static(static_visualizer) => {
                    RuntimeVisualizer::from_static(*static_visualizer, ty)
                }
                VisualizerSource::Custom { ref stmts } => RuntimeVisualizer::Interpreted {
                    stmts: stmts.clone(),
                    instruction_sheet: new_visual_instruction_sheet(db.compile_time(), stmts),
                },
            },
            None => RuntimeVisualizer::from_static(TRIVIAL_VISUALIZER, ty),
        },
        _ => todo!(),
    })
}
