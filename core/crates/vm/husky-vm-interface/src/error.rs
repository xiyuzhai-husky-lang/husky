use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __VMError {
    message: String,
    variant: __VMErrorVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum __VMErrorVariant {
    Normal,
}

impl __VMError {
    pub fn new_normal(message: String) -> __VMError {
        __VMError {
            message,
            variant: __VMErrorVariant::Normal,
        }
    }
}

impl Default for __VMErrorVariant {
    fn default() -> Self {
        __VMErrorVariant::Normal
    }
}

impl std::fmt::Display for __VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type __VMResult<T> = Result<T, __VMError>;
