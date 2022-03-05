use vm::{PrimitiveValue, StackValue, VMError, VMResult};

use crate::TokenProps;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceStackValue {
    Primitive(PrimitiveValue),
}

impl From<PrimitiveValue> for TraceStackValue {
    fn from(value: PrimitiveValue) -> Self {
        Self::Primitive(value)
    }
}

impl TraceStackValue {
    pub fn as_primitive(self) -> VMResult<PrimitiveValue> {
        match self {
            TraceStackValue::Primitive(value) => Ok(value),
            _ => todo!(),
        }
    }
}

impl From<StackValue<'static, 'static>> for TraceStackValue {
    fn from(_: StackValue<'static, 'static>) -> Self {
        todo!()
    }
}
