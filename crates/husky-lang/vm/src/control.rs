use crate::*;

#[derive(Debug, Clone)]
pub enum VMControl<'eval> {
    None,
    Return(EvalValue<'eval>),
    Break,
    Err(VMError),
}

impl<'eval> From<VMResult<()>> for VMControl<'eval> {
    fn from(result: VMResult<()>) -> Self {
        match result {
            Ok(_) => Self::None,
            Err(e) => Self::Err(e),
        }
    }
}

impl<'eval> From<VMResult<VMControl<'eval>>> for VMControl<'eval> {
    fn from(result: VMResult<VMControl<'eval>>) -> Self {
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
            VMControl::Err(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ControlSnapshot<'eval> {
    None,
    Return(StackValueSnapshot<'eval>),
    Break,
    Err(VMError),
}
