use latex_prelude::mode::LxMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LxCommandParameter {
    mode: LxMode,
}

impl LxCommandParameter {
    pub fn new(mode: LxMode) -> Self {
        Self { mode }
    }
}

pub type LxCommandParameters = Vec<LxCommandParameter>;
