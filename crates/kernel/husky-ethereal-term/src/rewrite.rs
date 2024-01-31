use super::*;

pub struct TermSubstitution {
    src: SymbolEtherealTerm,
    dst: EtherealTerm,
}

impl TermSubstitution {
    pub fn src(&self) -> SymbolEtherealTerm {
        self.src
    }

    pub fn dst(&self) -> EtherealTerm {
        self.dst
    }
}
