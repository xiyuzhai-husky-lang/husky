mod query;
mod ty;

pub use query::*;
pub use ty::*;

use avec::Avec;
use husky_eager_semantics::FuncStmt;
use husky_entity_route_syntax::EntityRoutePtr;
use husky_lazy_semantics::LazyStmt;
use husky_trace_protocol::*;
use print_utils::p;
use std::sync::Arc;
use visual_syntax::{StaticVisualTy, StaticVisualizer, StaticVisualizerVariant};
use vm::*;
use word::RootIdentifier;

#[derive(Clone)]
pub struct Visualizer {
    pub ty: VisualTy,
    pub variant: VisualizerVariant,
}

#[derive(Clone)]
pub enum VisualizerVariant {
    Compiled {
        call: for<'temp, 'eval> fn(&(dyn AnyValueDyn<'eval> + 'temp)) -> VisualData,
    },
    Vec {
        ty: EntityRoutePtr,
    },
    CyclicSlice {
        ty: EntityRoutePtr,
    },
    Custom {
        stmts: Avec<LazyStmt>,
    },
    Todo,
}

impl Visualizer {
    pub fn from_static(
        db: &dyn VisualizerQueryGroup,
        static_visualizer: &StaticVisualizer,
        ty: EntityRoutePtr,
    ) -> Visualizer {
        Visualizer {
            ty: VisualTy::from_static(db, ty, static_visualizer.ty),
            variant: match static_visualizer.variant {
                StaticVisualizerVariant::Compiled { call } => VisualizerVariant::Compiled { call },
                StaticVisualizerVariant::Vec => VisualizerVariant::Vec { ty },
                StaticVisualizerVariant::CyclicSlice => VisualizerVariant::CyclicSlice { ty },
            },
        }
    }

    // deprecated
    // fn visualize<'a, 'temp, 'eval>(
    //     &self,
    //     db: &dyn VisualizerQueryGroup,
    //     value: &(dyn AnyValueDyn<'eval> + 'temp),
    //     verbose: bool,
    // ) -> VisualData {
    //     match self.variant {
    //         VisualizerVariant::Compiled { call } => call(value),
    //         VisualizerVariant::Custom { .. } => {
    //             todo!()
    //             //     match eval_fast(
    //             //     db.upcast(),
    //             //     Some(instruction_sheet),
    //             //     None,
    //             //     vec![Ok(TempValue::TempRefTemp(value))].into_iter(),
    //             //     [].into_iter(),
    //             //     verbose,
    //             // ) {
    //             //     Ok(value) => value.owned().unwrap().take::<VisualData>().unwrap(),
    //             //     Err(_) => todo!(),
    //             // }
    //         }
    //         VisualizerVariant::Vec { ty, .. } => {
    //             let elem_ty = ty.spatial_arguments[0].take_entity_route();
    //             let elem_visualizer = db.visualizer(elem_ty);
    //             let virtual_vec: &VirtualVec<'eval> = value.downcast_ref();
    //             VisualData::Group(
    //                 virtual_vec
    //                     .iter()
    //                     .map(|elem| elem_visualizer.visualize(db, elem.any_ref(), verbose))
    //                     .collect(),
    //             )
    //         }
    //         VisualizerVariant::CyclicSlice { ty } => {
    //             let elem_ty = ty.spatial_arguments[0].take_entity_route();
    //             let elem_visualizer = db.visualizer(elem_ty);
    //             let virtual_cyclic_slice: &VirtualCyclicSlice<'eval> =
    //                 value.downcast_ref();
    //             VisualData::Group(
    //                 virtual_cyclic_slice
    //                     .iter()
    //                     .map(|elem| elem_visualizer.visualize(db, elem.any_ref(), verbose))
    //                     .collect(),
    //             )
    //         }
    //         VisualizerVariant::Todo => todo!(),
    //     }
    // }
}

impl std::fmt::Debug for Visualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.variant {
            VisualizerVariant::Compiled { call } => f.write_str("Compiled"),
            VisualizerVariant::Custom { .. } => f.write_str("Interpreted"),
            VisualizerVariant::Vec { .. } => f.write_str("Vec"),
            VisualizerVariant::CyclicSlice { ty } => f.write_str("CyclicSlice"),
            VisualizerVariant::Todo => f.write_str("Todo"),
        }
    }
}

impl PartialEq for Visualizer {
    fn eq(&self, other: &Self) -> bool {
        match (&self.variant, &other.variant) {
            (
                VisualizerVariant::Compiled { call: call0 },
                VisualizerVariant::Compiled { call: call1 },
            ) => {
                let call0: *const u8 = *call0 as *const u8;
                let call1: *const u8 = *call1 as *const u8;
                call0 == call1
            }
            (VisualizerVariant::Custom { .. }, VisualizerVariant::Custom { .. }) => todo!(),
            _ => false,
        }
    }
}

impl Eq for Visualizer {}
