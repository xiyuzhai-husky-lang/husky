use super::*;

pub struct DeclarativeTermSubstitution {
    src: RuneDeclarativeTerm,
    dst: DeclarativeTerm,
}

impl DeclarativeTermSubstitution {
    pub fn new(src: RuneDeclarativeTerm, dst: DeclarativeTerm) -> Self {
        Self { src, dst }
    }

    pub(crate) fn src(&self) -> RuneDeclarativeTerm {
        self.src
    }

    pub(crate) fn dst(&self) -> DeclarativeTerm {
        self.dst
    }
}
