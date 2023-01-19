use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermSymbol {
    idx: u8,
    kind: TermSymbolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermSymbolKind {
    Type0,
    Lifetime,
    Binding,
    Usize,
    Parameter,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct TermSymbolRegistry {
    next_ty0: u8,
    next_lifetime: u8,
    next_binding: u8,
    next_usize: u8,
    next_parameter: u8,
}

#[test]
fn term_symbol_registry_size_is_small() {
    assert_eq!(
        std::mem::size_of::<TermSymbolRegistry>(),
        5 * std::mem::size_of::<u8>()
    )
}

impl TermSymbolRegistry {
    pub fn new_ty0(&mut self) -> TermSymbol {
        let idx = self.next_ty0;
        self.next_ty0 += 1;
        TermSymbol {
            idx,
            kind: TermSymbolKind::Type0,
        }
    }
    pub fn new_lifetime(&mut self) -> TermSymbol {
        let idx = self.next_lifetime;
        self.next_lifetime += 1;
        TermSymbol {
            idx,
            kind: TermSymbolKind::Lifetime,
        }
    }
    pub fn new_binding(&mut self) -> TermSymbol {
        let idx = self.next_binding;
        self.next_binding += 1;
        TermSymbol {
            idx,
            kind: TermSymbolKind::Binding,
        }
    }
    pub fn new_usize(&mut self) -> TermSymbol {
        let idx = self.next_usize;
        self.next_usize += 1;
        TermSymbol {
            idx,
            kind: TermSymbolKind::Usize,
        }
    }
    pub fn new_parameter(&mut self) -> TermSymbol {
        let idx = self.next_parameter;
        self.next_parameter += 1;
        TermSymbol {
            idx,
            kind: TermSymbolKind::Parameter,
        }
    }
}
