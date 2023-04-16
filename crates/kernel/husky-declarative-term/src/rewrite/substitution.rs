use super::*;

pub struct DeclarativeTermSubstitution {
    src: DeclarativeTermPlaceholder,
    dst: DeclarativeTerm,
}

impl DeclarativeTermSubstitution {
    pub fn src(&self) -> DeclarativeTermPlaceholder {
        self.src
    }

    pub fn dst(&self) -> DeclarativeTerm {
        self.dst
    }
}
