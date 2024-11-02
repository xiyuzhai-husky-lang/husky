use crate::precedence::VdPrecedenceRange;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VdBasePrefixOpr {}

impl VdBasePrefixOpr {
    pub fn latex_code(self) -> &'static str {
        todo!()
    }

    pub fn precedence_range(self) -> VdPrecedenceRange {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VdCompositePrefixOpr {
    /// `d/dx`
    Differential,
}

impl VdCompositePrefixOpr {
    pub fn latex_code(self) -> &'static str {
        todo!()
    }
}
