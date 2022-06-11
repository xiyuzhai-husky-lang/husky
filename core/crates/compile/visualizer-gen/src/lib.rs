mod query;

pub use query::*;

use avec::Avec;
use entity_route::EntityRoutePtr;
use husky_tracer_protocol::*;
use print_utils::p;
use semantics_eager::FuncStmt;
use static_defn::CyclicSlice;
use std::sync::Arc;
use visual_syntax::StaticVisualizer;
use vm::*;

#[derive(Clone)]
pub enum Visualizer {
    Compiled(for<'temp, 'eval> fn(&(dyn AnyValueDyn<'eval> + 'temp)) -> VisualProps),
    Vec {
        ty: EntityRoutePtr,
    },
    CyclicSlice {
        ty: EntityRoutePtr,
    },
    Interpreted {
        stmts: Avec<FuncStmt>,
        instruction_sheet: Arc<InstructionSheet>,
    },
    Todo,
}

impl Visualizer {
    pub fn visualize<'a, 'temp, 'eval>(
        &self,
        db: &dyn VisualizerQueryGroup,
        value: &(dyn AnyValueDyn<'eval> + 'temp),
        verbose: bool,
    ) -> VisualProps {
        match self {
            Visualizer::Compiled(compiled) => compiled(value),
            Visualizer::Interpreted {
                instruction_sheet, ..
            } => match eval_fast(
                db.upcast(),
                Some(instruction_sheet),
                None,
                vec![Ok(TempValue::TempRefTemp(value))].into_iter(),
                [].into_iter(),
                verbose,
            ) {
                Ok(value) => value.owned().unwrap().take::<VisualProps>().unwrap(),
                Err(_) => todo!(),
            },
            Visualizer::Vec { ty, .. } => {
                let elem_ty = ty.spatial_arguments[0].take_entity_route();
                let elem_visualizer = db.visualizer(elem_ty);
                let virtual_vec: &Vec<MemberValue<'eval>> = value.downcast_ref();
                VisualProps::Group(
                    virtual_vec
                        .iter()
                        .map(|elem| elem_visualizer.visualize(db, elem.any_ref(), verbose))
                        .collect(),
                )
            }
            Visualizer::CyclicSlice { ty } => {
                let elem_ty = ty.spatial_arguments[0].take_entity_route();
                let elem_visualizer = db.visualizer(elem_ty);
                let virtual_cyclic_slice: &CyclicSlice<'eval, MemberValue<'eval>> =
                    value.downcast_ref();
                VisualProps::Group(
                    virtual_cyclic_slice
                        .iter()
                        .map(|elem| elem_visualizer.visualize(db, elem.any_ref(), verbose))
                        .collect(),
                )
            }
            Visualizer::Todo => todo!(),
        }
    }
}

impl std::fmt::Debug for Visualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visualizer::Compiled(arg0) => f.write_str("Compiled"),
            Visualizer::Interpreted { .. } => f.write_str("Interpreted"),
            Visualizer::Vec { .. } => f.write_str("Vec"),
            Visualizer::CyclicSlice { ty } => f.write_str("CyclicSlice"),
            Visualizer::Todo => f.write_str("Todo"),
        }
    }
}

impl PartialEq for Visualizer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Visualizer::Compiled(l0), Visualizer::Compiled(r0)) => {
                let l0: *const u8 = *l0 as *const u8;
                let r0: *const u8 = *r0 as *const u8;
                l0 == r0
            }
            (
                Visualizer::Interpreted {
                    instruction_sheet: instruction_sheet0,
                    ..
                },
                Visualizer::Interpreted {
                    instruction_sheet: instruction_sheet1,
                    ..
                },
            ) => todo!(),
            _ => false,
        }
    }
}

impl Eq for Visualizer {}

impl Visualizer {
    pub fn from_static(static_visualizer: StaticVisualizer, ty: EntityRoutePtr) -> Visualizer {
        match static_visualizer {
            StaticVisualizer::Compiled(compiled) => Visualizer::Compiled(compiled),
            StaticVisualizer::Vec => Visualizer::Vec { ty },
            StaticVisualizer::CyclicSlice => Visualizer::CyclicSlice { ty },
        }
    }
}
