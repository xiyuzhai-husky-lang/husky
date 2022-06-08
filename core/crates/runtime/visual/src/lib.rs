mod query;

use avec::Avec;
use print_utils::p;
pub use query::*;

use compile_time_db::*;
use entity_route::EntityRoutePtr;
use semantics_eager::FuncStmt;
use static_defn::CyclicSlice;
use std::sync::Arc;
use visual_syntax::StaticVisualizer;
use vm::*;

#[derive(Clone)]
pub enum RuntimeVisualizer {
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

impl RuntimeVisualizer {
    pub fn visualize<'a, 'temp, 'eval>(
        &self,
        db: &dyn RuntimeVisualizerQueryGroup,
        value: &(dyn AnyValueDyn<'eval> + 'temp),
        verbose: bool,
    ) -> VisualProps {
        match self {
            RuntimeVisualizer::Compiled(compiled) => compiled(value),
            RuntimeVisualizer::Interpreted {
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
            RuntimeVisualizer::Vec { ty, .. } => {
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
            RuntimeVisualizer::CyclicSlice { ty } => {
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
            RuntimeVisualizer::Todo => todo!(),
        }
    }
}

impl std::fmt::Debug for RuntimeVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeVisualizer::Compiled(arg0) => f.write_str("Compiled"),
            RuntimeVisualizer::Interpreted { .. } => f.write_str("Interpreted"),
            RuntimeVisualizer::Vec { .. } => f.write_str("Vec"),
            RuntimeVisualizer::CyclicSlice { ty } => f.write_str("CyclicSlice"),
            RuntimeVisualizer::Todo => f.write_str("Todo"),
        }
    }
}

impl PartialEq for RuntimeVisualizer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuntimeVisualizer::Compiled(l0), RuntimeVisualizer::Compiled(r0)) => {
                let l0: *const u8 = *l0 as *const u8;
                let r0: *const u8 = *r0 as *const u8;
                l0 == r0
            }
            (
                RuntimeVisualizer::Interpreted {
                    instruction_sheet: instruction_sheet0,
                    ..
                },
                RuntimeVisualizer::Interpreted {
                    instruction_sheet: instruction_sheet1,
                    ..
                },
            ) => todo!(),
            _ => false,
        }
    }
}

impl Eq for RuntimeVisualizer {}

impl RuntimeVisualizer {
    pub fn from_static(
        static_visualizer: StaticVisualizer,
        ty: EntityRoutePtr,
    ) -> RuntimeVisualizer {
        match static_visualizer {
            StaticVisualizer::Compiled(compiled) => RuntimeVisualizer::Compiled(compiled),
            StaticVisualizer::Vec => RuntimeVisualizer::Vec { ty },
            StaticVisualizer::CyclicSlice => RuntimeVisualizer::CyclicSlice { ty },
        }
    }
}
