pub mod parameter;
pub mod table;

use self::parameter::LxCommandParameters;
use crate::path::LxCommandPath;
use parameter::LxCommandParameter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LxCommandSignature {
    path: LxCommandPath,
    parameters: LxCommandParameters,
}

impl LxCommandSignature {
    pub fn path(&self) -> LxCommandPath {
        self.path
    }

    pub fn parameters(&self) -> &[LxCommandParameter] {
        &self.parameters
    }
}
