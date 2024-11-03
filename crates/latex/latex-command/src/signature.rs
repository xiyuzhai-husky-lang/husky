pub mod parameter;
pub mod table;

use self::parameter::LxCommandParameters;
use crate::path::LxCommandPath;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LxCommandSignature {
    path: LxCommandPath,
    parameters: LxCommandParameters,
}
