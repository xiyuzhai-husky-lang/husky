use super::*;

pub struct DecTermSubstitution {
    src: DecRune,
    dst: DecTerm,
}

impl DecTermSubstitution {
    pub fn new(src: DecRune, dst: DecTerm) -> Self {
        Self { src, dst }
    }

    pub(crate) fn src(&self) -> DecRune {
        self.src
    }

    pub(crate) fn dst(&self) -> DecTerm {
        self.dst
    }
}
