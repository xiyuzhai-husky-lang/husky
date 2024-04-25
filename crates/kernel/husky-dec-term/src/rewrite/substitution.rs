use super::*;

pub struct DecTermSubstitution {
    src: DecLambdaVariable,
    dst: DecTerm,
}

impl DecTermSubstitution {
    pub fn new(src: DecLambdaVariable, dst: DecTerm) -> Self {
        Self { src, dst }
    }

    pub(crate) fn src(&self) -> DecLambdaVariable {
        self.src
    }

    pub(crate) fn dst(&self) -> DecTerm {
        self.dst
    }
}
