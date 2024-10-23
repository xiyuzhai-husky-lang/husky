use latex_prelude::mode::LxMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LxCommandParameter {
    mode: LxMode,
}

pub type LxCommandParameters = Vec<LxCommandParameter>;
