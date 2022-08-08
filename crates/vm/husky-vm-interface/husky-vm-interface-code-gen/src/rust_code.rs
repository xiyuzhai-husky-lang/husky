use super::PRIMITIVE_TYPES;
use crate::NONPRIMITIVE_BUILTIN_TYPES;
use husky_print_utils::p;
use husky_rust_code_repr::{
    registration::NonPrimitiveTypeRegistration, registration::RootPrimitiveTypeRegistration,
    BuildCodeGenStart,
};
use husky_write_utils::w;
use std;
use std::fs::File;
use std::io::prelude::*;

pub(crate) fn write_rust_code(rust_path: &str) -> std::io::Result<()> {
    let mut f = File::create(rust_path).expect(&format!("rust path {rust_path} doesn't exist"));
    w!(f; BuildCodeGenStart);
    for ty in PRIMITIVE_TYPES {
        w!(f; RootPrimitiveTypeRegistration { ty })
    }
    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        w!(f; NonPrimitiveTypeRegistration { ty })
    }
    Ok(())
}
