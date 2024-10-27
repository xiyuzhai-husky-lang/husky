use crate::{parameter::LxCommandParameters, path::LxCommandPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LxCommand {
    path: LxCommandPath,
    parameters: LxCommandParameters,
}
