#![feature(const_fn_trait_bound, const_fn_fn_ptr_basics)]
use husky_entity_route_syntax::EntityRoutePtr;
use husky_trace_protocol::*;
use vm::*;

pub struct StaticVisualizer {
    pub ty: StaticVisualTy,
    pub variant: StaticVisualizerVariant,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StaticVisualTy {
    Void,
    Bool,
    B32,
    B64,
    I32,
    F32,
    Group,
    Point2d,
    Shape2d,
    Region2d,
    Image2d,
    Graphics2d,
    Dataset,
}

#[derive(Clone)]
pub enum StaticVisualizerVariant {
    Compiled {
        call: for<'temp, 'eval> fn(&(dyn AnyValueDyn<'eval> + 'temp)) -> VisualData,
    },
    Vec,
    CyclicSlice,
}

impl std::fmt::Debug for StaticVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("StaticVisualizer")
    }
}

impl PartialEq for StaticVisualizer {
    fn eq(&self, other: &Self) -> bool {
        if self.ty != other.ty {
            return false;
        }
        match (&self.variant, &other.variant) {
            (
                StaticVisualizerVariant::Compiled { call: f0 },
                StaticVisualizerVariant::Compiled { call: f1 },
            ) => {
                let f0 = *f0 as usize;
                let f1 = *f1 as usize;
                f0 == f1
            }
            (StaticVisualizerVariant::Vec, StaticVisualizerVariant::Vec) => true,
            _ => false,
        }
    }
}

impl Eq for StaticVisualizer {}

pub const fn primitive_visualizer(ty: StaticVisualTy) -> StaticVisualizer {
    StaticVisualizer {
        ty,
        variant: StaticVisualizerVariant::Compiled {
            call: visualize_primitive,
        },
    }
}

fn visualize_primitive<'temp, 'eval>(value: &(dyn AnyValueDyn<'eval> + 'temp)) -> VisualData {
    match value.ty_dyn() {
        EntityRoutePtr::Root(_) => VisualData::Primitive {
            value: value.take_copyable_dyn().into(),
        },
        EntityRoutePtr::Custom(_) => VisualData::Primitive { value: ().into() },
        EntityRoutePtr::ThisType => panic!(),
    }
}
