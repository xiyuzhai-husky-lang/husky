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

impl Default for StaticVisualTy {
    fn default() -> Self {
        StaticVisualTy::Void
    }
}
