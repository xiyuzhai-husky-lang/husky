#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VdSpaceAnnotation {
    Apply(LxApplyAnnotation) = 1,
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
