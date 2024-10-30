#[enum_class::from_variants]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum LxTokenAnnotation {
    #[default]
    Void,
    Integral(LxIntegralAnnotation),
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
