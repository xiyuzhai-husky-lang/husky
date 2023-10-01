#![feature(extern_types)]

use husky_trace_protocol_old::VisualData;
use husky_vm_interface::{__Register, __VMResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StaticVisualTy {
    Void,
    Bool,
    B32,
    B64,
    Integer,
    Float,
    Group,
    Point2d,
    Shape2d,
    Region2d,
    Image2d,
    Graphics2d,
    Dataset,
    ThickFp,
}

impl Default for StaticVisualTy {
    fn default() -> Self {
        StaticVisualTy::Void
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StaticVisualizer {
    pub visual_ty: StaticVisualTy,
    pub fp: StaticVisualizerFp,
}

#[derive(Clone, Copy)]
pub struct StaticVisualizerFp(pub for<'eval> fn(&__Register<'eval>) -> __VMResult<VisualData>);

impl std::fmt::Debug for StaticVisualizerFp {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PartialEq for StaticVisualizerFp {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}

impl Eq for StaticVisualizerFp {}
