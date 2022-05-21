use crate::*;
use entity_syntax::EntityLocus;
use instruction_gen::new_visual_instruction_sheet;
use semantics_entity::EntityDefnVariant;
use static_defn::EntityStaticDefnVariant;
use visual_semantics::VisualizerSource;
use visual_syntax::TRIVIAL_VISUALIZER;

#[salsa::query_group(VisualQueryGroupStorage)]
pub trait VisualQueryGroup: AskCompileTime {
    fn visualizer(&self, version: usize, ty: EntityRoutePtr) -> Arc<RuntimeVisualizer>;
}

fn visualizer(
    db: &dyn VisualQueryGroup,
    version: usize,
    ty: EntityRoutePtr,
) -> Arc<RuntimeVisualizer> {
    let ty_defn = db.compile_time().entity_defn(ty).unwrap();
    Arc::new(match ty_defn.variant {
        EntityDefnVariant::Type {
            ref opt_visualizer_source,
            ..
        } => match opt_visualizer_source {
            Some(visualizer_source) => match visualizer_source {
                VisualizerSource::Static(static_visualizer) => static_visualizer.into(),
                VisualizerSource::Xml {
                    ref stmts,
                    ref xml_expr,
                } => RuntimeVisualizer::Interpreted {
                    stmts: stmts.clone(),
                    xml_expr: xml_expr.clone(),
                    instruction_sheet: new_visual_instruction_sheet(
                        db.compile_time(),
                        stmts,
                        xml_expr,
                    ),
                },
            },
            None => RuntimeVisualizer::Compiled(TRIVIAL_VISUALIZER.compiled),
        },
        _ => todo!(),
    })
}
