use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermParameter {
    idx: u8,
    ty: Term,
}

impl TermParameter {
    pub fn new(idx: u8) -> Self {
        Self { idx }
    }
}
