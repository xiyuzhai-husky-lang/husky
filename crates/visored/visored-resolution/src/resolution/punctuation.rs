use visored_opr::{opr::VdBaseOpr, separator::VdBaseSeparator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdPunctuationResolution {
    Opr(VdBaseOpr),
    Separator(VdBaseSeparator),
    Todo,
}

impl VdPunctuationResolution {
    pub const SEPARATOR_ADD: Self = Self::Separator(VdBaseSeparator::ADD);
    pub const EQ: Self = Self::Opr(VdBaseOpr::EQ);
}
