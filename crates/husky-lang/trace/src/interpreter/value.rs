use vm::{PrimitiveValue, VMError, VMResult};

use crate::TokenProps;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceStackValue {
    Primitive(PrimitiveValue),
    Err(VMError),
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
