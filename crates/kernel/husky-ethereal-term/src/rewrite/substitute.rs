use super::*;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EtherealTermSubstitution {
    src: RuneEtherealTerm,
    dst: EtherealTerm,
}

impl EtherealTermSubstitution {
    pub fn new(src: RuneEtherealTerm, dst: EtherealTerm) -> Self {
        Self { src, dst }
    }
}

/// # getters
impl EtherealTermSubstitution {
    pub fn src(&self) -> RuneEtherealTerm {
        self.src
    }

    pub fn dst(&self) -> EtherealTerm {
        self.dst
    }
}
