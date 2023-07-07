use super::*;

pub struct DeclarativeTermSubstitution {
    src: DeclarativeTermVariable,
    dst: DeclarativeTerm,
}

impl DeclarativeTermSubstitution {
    pub fn new(src: DeclarativeTermVariable, dst: DeclarativeTerm) -> Self {
        Self { src, dst }
    }

    pub(crate) fn src(&self) -> DeclarativeTermVariable {
        self.src
    }

    pub(crate) fn dst(&self) -> DeclarativeTerm {
        self.dst
    }
}
