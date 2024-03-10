use super::*;
use husky_print_utils::p;
use std::num::ParseFloatError;

/// allowing representing very large number
#[salsa::interned(jar = TermPreludeJar)]
pub struct F32Literal {
    pub value: OrderedFloat<f32>,
    #[return_ref]
    pub text: String,
}

impl F32Literal {
    pub fn try_new(mut text: String, db: &::salsa::Db) -> Result<Self, ParseFloatError> {
        if !text.ends_with("f32") {
            if text.ends_with(".") {
                text += "0f32"
            } else {
                text += "f32"
            }
        }
        let value: OrderedFloat<f32> = match text.trim_end_matches("f32").parse() {
            Ok(value) => value,
            Err(e) => {
                p!(text, e);
                todo!()
            }
        };
        Ok(Self::new(db, value, text))
    }
}

/// allowing representing very large number
#[salsa::interned(jar = TermPreludeJar)]
pub struct F64Literal {
    pub value: OrderedFloat<f64>,
    #[return_ref]
    pub text: String,
}

impl F64Literal {
    pub fn try_new(mut text: String, db: &::salsa::Db) -> Result<Self, ParseFloatError> {
        if !text.ends_with("f64") {
            if text.ends_with(".") {
                text += "0f64"
            } else {
                text += "f64"
            }
        }
        let value: OrderedFloat<f64> = match text.trim_end_matches("f64").parse() {
            Ok(value) => value,
            Err(e) => {
                p!(text, e);
                todo!()
            }
        };
        Ok(Self::new(db, value, text))
    }
}
