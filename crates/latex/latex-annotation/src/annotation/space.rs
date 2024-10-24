#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxSpaceAnnotation {
    Void,
    Apply(LxApplyAnnotation),
    Sever(LxSeverAnnotation),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxApplyAnnotation {
    ScalarMul,
    ScalarDifferentialFormMul,
    Differentiation,
    Integration,
}

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxSeverAnnotation {
    Index,
}
