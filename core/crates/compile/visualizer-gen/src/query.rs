use crate::*;
use entity_syntax::EntityLocus;
use instruction_gen::{new_visual_instruction_sheet, InstructionGenQueryGroup};
use print_utils::p;
use semantics_entity::{EntityDefnQueryGroup, EntityDefnVariant};
use static_defn::EntityStaticDefnVariant;
use upcast::Upcast;
use visual_semantics::VisualizerSource;
use visual_syntax::TRIVIAL_VISUALIZER;

#[salsa::query_group(VisualizerQueryGroupStorage)]
pub trait VisualizerQueryGroup:
    EntityDefnQueryGroup + Upcast<dyn InterpreterQueryGroup> + Upcast<dyn InstructionGenQueryGroup>
{
    fn visualizer(&self, ty: EntityRoutePtr) -> Arc<Visualizer>;
}

fn visualizer(db: &dyn VisualizerQueryGroup, ty: EntityRoutePtr) -> Arc<Visualizer> {
    let ty_defn = db.entity_defn(ty).unwrap();
    Arc::new(match ty_defn.variant {
        EntityDefnVariant::Ty {
            ref opt_visualizer_source,
            ..
        } => match opt_visualizer_source {
            Some(visualizer_source) => match visualizer_source {
                VisualizerSource::Static(static_visualizer) => {
                    Visualizer::from_static(db, static_visualizer, ty)
                }
                VisualizerSource::Custom { ref stmts } => Visualizer {
                    ty: VisualTy::from_stmts(stmts),
                    variant: VisualizerVariant::Custom {
                        stmts: stmts.clone(),
                    },
                },
            },
            None => Visualizer::from_static(db, &TRIVIAL_VISUALIZER, ty),
        },
        _ => todo!(),
    })
}
