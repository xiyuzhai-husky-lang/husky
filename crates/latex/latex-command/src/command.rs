use crate::{parameter::TexCommandParameters, path::TexCommandPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TexCommand {
    path: TexCommandPath,
    parameters: TexCommandParameters,
}
