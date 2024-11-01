use crate::precedence::VdPrecedenceRange;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VdBaseSuffixOpr {}

impl VdBaseSuffixOpr {
    pub fn fmt_str(self) -> &'static str {
        todo!()
    }

    pub fn precedence_range(self) -> VdPrecedenceRange {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VdCompositeSuffixOpr {}
