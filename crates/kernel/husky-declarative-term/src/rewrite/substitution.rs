use super::*;

pub struct DeclarativeTermSubstitution {
    src: DeclarativeTermRune,
    dst: DeclarativeTerm,
}

impl DeclarativeTermSubstitution {
    pub fn new(src: DeclarativeTermRune, dst: DeclarativeTerm) -> Self {
        Self { src, dst }
    }

    pub(crate) fn src(&self) -> DeclarativeTermRune {
        self.src
    }

    pub(crate) fn dst(&self) -> DeclarativeTerm {
        self.dst
    }
}
