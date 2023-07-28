use crate::*;

#[derive(Debug, Clone)]
pub enum VMControl {
    None,
    Return(__RegularValue),
    Break,
    Err(__VMError),
}

impl Default for VMControl {
    fn default() -> Self {
        VMControl::None
    }
}

impl From<__VMResult<()>> for VMControl {
    fn from(result: __VMResult<()>) -> Self {
        match result {
            Ok(_) => Default::default(),
            Err(e) => Self::Err(e),
        }
    }
}

impl From<__VMResult<VMControl>> for VMControl {
    fn from(result: __VMResult<VMControl>) -> Self {
        match result {
            Ok(control) => control,
            Err(e) => Self::Err(e),
        }
    }
}

impl VMControl {
    pub(crate) fn snapshot(&self) -> ControlSnapshot {
        match self {
            VMControl::None => ControlSnapshot::None,
            VMControl::Return(value) => ControlSnapshot::Return(value.snapshot()),
            VMControl::Break => ControlSnapshot::Break,
            VMControl::Err(e) => ControlSnapshot::Err(e.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ControlSnapshot {
    None,
    Return(__RegularValue),
    Break,
    Err(__VMError),
}
