#[derive(Debug, PartialEq, Eq)]
pub enum ExprTypeError {
    Original(OriginalExprTypeError),
    Derived(DerivedExprTypeError),
}

impl From<DerivedExprTypeError> for ExprTypeError {
    fn from(v: DerivedExprTypeError) -> Self {
        Self::Derived(v)
    }
}

impl From<OriginalExprTypeError> for ExprTypeError {
    fn from(v: OriginalExprTypeError) -> Self {
        Self::Original(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OriginalExprTypeError {}

#[derive(Debug, PartialEq, Eq)]
pub enum DerivedExprTypeError {
    TypeInfoErr,
    ExprError,
    EntityTypeError,
    BoxListApplicationFirstArgumentError,
    ApplicationArgumentTypeNotInferred,
    PrefixOperandTypeNotInferred,
    BinaryOpnFirstArgumentTypeNotInferred,
    BinaryOpnSecondArgumentTypeNotInferred,
}

pub type ExprTypeResult<T> = Result<T, ExprTypeError>;
pub type ExprTypeResultRef<'a, T> = Result<T, &'a ExprTypeError>;
