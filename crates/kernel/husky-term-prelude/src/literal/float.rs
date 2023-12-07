use super::*;
use std::num::ParseFloatError;

/// allowing representing very large number
#[salsa::interned(jar = TermPreludeJar)]
pub struct TermF32Literal {
    pub value: OrderedFloat<f64>,
    #[return_ref]
    pub text: String,
}

impl TermF32Literal {
    pub fn try_new(mut text: String, db: &::salsa::Db) -> Result<Self, ParseFloatError> {
        if !text.ends_with("f32") {
            text += "f32"
        }
        let value: OrderedFloat<f64> = text.parse()?;
        Ok(Self::new(db, value, text))
    }
}

/// allowing representing very large number
#[salsa::interned(jar = TermPreludeJar)]
pub struct TermF64Literal {
    pub value: OrderedFloat<f64>,
    #[return_ref]
    pub text: String,
}
