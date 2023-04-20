use super::*;

pub struct DeclarativeTermSubstitution {
    src: DeclarativeTermVariable,
    dst: DeclarativeTerm,
}

impl DeclarativeTermSubstitution {
    pub fn src(&self) -> DeclarativeTermVariable {
        self.src
    }

    pub fn dst(&self) -> DeclarativeTerm {
        self.dst
    }
}
