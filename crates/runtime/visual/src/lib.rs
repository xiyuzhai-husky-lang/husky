mod query;

use avec::Avec;
use print_utils::p;
pub use query::*;

use compile_time_db::*;
use entity_route::EntityRoutePtr;
use semantics_eager::FuncStmt;
use std::sync::Arc;
use visual_syntax::{StaticVisualizer, VisualProps};
use vm::{
    eval_fast, AnyValueDyn, InstructionSheet, MemberValue, TempValue, VMRuntimeResult, XmlValue,
};

#[derive(Clone)]
pub enum RuntimeVisualizer {
    Compiled(for<'eval> fn(&(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps),
    Vec {
        ty: EntityRoutePtr,
    },
    Interpreted {
        stmts: Avec<FuncStmt>,
        instruction_sheet: Arc<InstructionSheet>,
    },
}

impl RuntimeVisualizer {
    pub fn visualize<'a, 'eval>(
        &self,
        db: &dyn VisualQueryGroup,
        value: &(dyn AnyValueDyn<'eval> + 'eval),
    ) -> VisualProps {
        match self {
            RuntimeVisualizer::Compiled(compiled) => compiled(value),
            RuntimeVisualizer::Interpreted {
                instruction_sheet, ..
            } => match eval_fast(
                db.compile_time(),
                Some(instruction_sheet),
                None,
                vec![Ok(TempValue::TempRefEval(value))].into_iter(),
                [].into_iter(),
            ) {
                Ok(value) => {
                    let xml_value: XmlValue = value.owned().unwrap().take().unwrap();
                    VisualProps::from_xml_value(xml_value)
                }
                Err(_) => todo!(),
            },
            RuntimeVisualizer::Vec { ty, .. } => {
                let elem_ty = ty.spatial_arguments[0].take_entity_route();
                let elem_visualizer = db.visualizer(elem_ty);
                let virtual_vec: &Vec<MemberValue<'eval>> = value.downcast_ref();
                VisualProps::Group(
                    virtual_vec
                        .iter()
                        .map(|elem| elem_visualizer.visualize(db, elem.any_ref()))
                        .collect(),
                )
            }
        }
    }
}

impl std::fmt::Debug for RuntimeVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeVisualizer::Compiled(arg0) => f.write_str("Compiled"),
            RuntimeVisualizer::Interpreted { .. } => f.write_str("Interpreted"),
            RuntimeVisualizer::Vec { .. } => f.write_str("Vec"),
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
        }
    }
}
