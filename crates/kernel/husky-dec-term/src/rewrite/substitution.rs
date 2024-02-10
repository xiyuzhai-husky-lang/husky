use super::*;

pub struct DecTermSubstitution {
    src: DecHvar,
    dst: DecTerm,
}

impl DecTermSubstitution {
    pub fn new(src: DecHvar, dst: DecTerm) -> Self {
        Self { src, dst }
    }

    pub(crate) fn src(&self) -> DecHvar {
        self.src
    }

    pub(crate) fn dst(&self) -> DecTerm {
        self.dst
    }
}
