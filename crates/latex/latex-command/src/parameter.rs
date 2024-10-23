use latex_prelude::mode::TexMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TexCommandParameter {
    mode: TexMode,
}

pub type TexCommandParameters = Vec<TexCommandParameter>;
