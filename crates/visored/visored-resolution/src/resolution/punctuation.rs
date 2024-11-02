use visored_opr::opr::VdBaseOpr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdPunctuationResolution {
    Opr(VdBaseOpr),
    Todo,
}

impl VdPunctuationResolution {
    pub const ADD: Self = Self::Opr(VdBaseOpr::ADD);
}
