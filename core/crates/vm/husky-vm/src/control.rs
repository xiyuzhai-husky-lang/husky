use crate::*;

#[derive(Debug, Clone)]
pub enum VMControl<'eval> {
    None,
    Return(__Register<'eval>),
    Break,
    Err(__VMError),
}

impl<'eval> From<__VMResult<()>> for VMControl<'eval> {
    fn from(result: __VMResult<()>) -> Self {
        match result {
            Ok(_) => Self::None,
            Err(e) => Self::Err(e),
        }
    }
}

impl<'eval> From<__VMResult<VMControl<'eval>>> for VMControl<'eval> {
    fn from(result: __VMResult<VMControl<'eval>>) -> Self {
        match result {
            Ok(control) => control,
            Err(e) => Self::Err(e),
        }
    }
}

impl<'eval> VMControl<'eval> {
    pub(crate) fn snapshot(&self) -> ControlSnapshot<'eval> {
        match self {
            VMControl::None => ControlSnapshot::None,
            VMControl::Return(value) => ControlSnapshot::Return(value.snapshot()),
            VMControl::Break => ControlSnapshot::Break,
            VMControl::Err(e) => ControlSnapshot::Err(e.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ControlSnapshot<'eval> {
    None,
    Return(__Register<'eval>),
    Break,
    Err(__VMError),
}
