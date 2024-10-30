#[enum_class::from_variants]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum VdSpaceAnnotation {
    #[default]
    Void,
    Apply(LxApplyAnnotation),
    Sever(LxSeverAnnotation),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxApplyAnnotation {
    ScalarMul,
    ScalarDifferentialFormMul,
}

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxSeverAnnotation {
    Index,
}
