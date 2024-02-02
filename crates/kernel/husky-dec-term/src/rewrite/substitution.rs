use super::*;

pub struct DecTermSubstitution {
    src: RuneDecTerm,
    dst: DecTerm,
}

impl DecTermSubstitution {
    pub fn new(src: RuneDecTerm, dst: DecTerm) -> Self {
        Self { src, dst }
    }

    pub(crate) fn src(&self) -> RuneDecTerm {
        self.src
    }

    pub(crate) fn dst(&self) -> DecTerm {
        self.dst
    }
}
