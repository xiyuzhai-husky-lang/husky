#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VdTokenAnnotation {
    Integral(LxIntegralAnnotation) = 1,
    Variable(LxVariableAnnotation),
    Differential,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxIntegralAnnotation {
    SingleVariableDefiniteIntegralOverReal,
    SingleVariableIndefiniteIntegralOverReal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxVariableAnnotation {
    Usage,
    SingleVariableIntegralVariableDecl,
}
