#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LxTokenLane {
    Main,
    Annotation(LxTokenAnnotationLane),
}

#[salsa::interned]
pub struct LxTokenAnnotationLane {
    pub parent: LxTokenLane,
}
