#[derive(Debug, PartialEq, Eq)]
pub enum LxTokenAnnotation {
    Void,
    Integral(LxIntegralAnnotation),
    Variable(LxVariableAnnotation),
}

#[derive(Debug, PartialEq, Eq)]
pub enum LxIntegralAnnotation {}

#[derive(Debug, PartialEq, Eq)]
pub enum LxVariableAnnotation {}
