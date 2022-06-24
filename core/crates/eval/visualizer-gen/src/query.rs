use crate::*;
use entity_syntax::EntityLocus;
use husky_compile_time::AskCompileTime;
use instruction_gen::{new_visual_instruction_sheet, InstructionGenQueryGroup};
use print_utils::p;
use semantics_entity::{EntityDefnQueryGroup, EntityDefnVariant};
use static_defn::EntityStaticDefnVariant;
use upcast::Upcast;
use visual_semantics::VisualizerSource;
use visual_syntax::primitive_visualizer;

#[salsa::query_group(VisualizerQueryGroupStorage)]
pub trait VisualizerQueryGroup:
    AskCompileTime + Upcast<dyn InterpreterQueryGroup> + Upcast<dyn InstructionGenQueryGroup>
{
    fn visualizer(&self, ty: EntityRoutePtr) -> Arc<Visualizer>;
    fn visual_ty(&self, ty: EntityRoutePtr) -> VisualTy;
}

fn visualizer(db: &dyn VisualizerQueryGroup, ty: EntityRoutePtr) -> Arc<Visualizer> {
    let ty_defn = db.compile_time().entity_defn(ty).unwrap();
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
                    ty: VisualTy::from_stmts(db, stmts),
                    variant: VisualizerVariant::Custom {
                        stmts: stmts.clone(),
                    },
                },
            },
            None => Visualizer::from_static(db, &primitive_visualizer(StaticVisualTy::Void), ty),
        },
        _ => todo!(),
    })
}

fn visual_ty(db: &dyn VisualizerQueryGroup, ty: EntityRoutePtr) -> VisualTy {
    db.visualizer(ty).ty
}
