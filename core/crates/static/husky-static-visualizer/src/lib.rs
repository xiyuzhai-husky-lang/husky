#![feature(extern_types)]

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
}

impl Default for StaticVisualTy {
    fn default() -> Self {
        StaticVisualTy::Void
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StaticVisualizer {
    pub visual_ty: StaticVisualTy,
}
