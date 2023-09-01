#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __VMError {
    message: String,
    variant: __VMErrorVariant,
}

impl From<(usize, __VMError)> for __VMError {
    fn from((sample_id, e): (usize, __VMError)) -> Self {
        match e.variant {
            __VMErrorVariant::Normal => Self {
                message: e.message,
                variant: __VMErrorVariant::FromBatch { sample_id },
            },
            __VMErrorVariant::FromBatch { sample_id: _ } => e,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum __VMErrorVariant {
    Normal,
    FromBatch { sample_id: usize },
}

impl __VMError {
    pub fn new_normal(message: String) -> __VMError {
        __VMError {
            message,
            variant: __VMErrorVariant::Normal,
        }
    }

    pub fn linkage_call_error(msg: &str) -> __VMError {
        __VMError {
            message: msg.to_string(),
            variant: __VMErrorVariant::Normal,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn variant(&self) -> &__VMErrorVariant {
        &self.variant
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = format!(
            r#"{message}      
-> {}"#,
            &self.message
        );
        self
    }
}

impl Default for __VMErrorVariant {
    fn default() -> Self {
        __VMErrorVariant::Normal
    }
}

impl std::fmt::Display for __VMError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type VMResult<T> = Result<T, __VMError>;
