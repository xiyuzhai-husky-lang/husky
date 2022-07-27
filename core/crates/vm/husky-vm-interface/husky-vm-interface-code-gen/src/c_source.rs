use super::PRIMITIVE_TYPES;
use crate::NONPRIMITIVE_BUILTIN_TYPES;
use husky_c_code_repr::*;
use std;
use std::fs::File;
use std::io::prelude::*;

pub fn write_c_source(c_source_path: &str) -> std::io::Result<()> {
    use std::fmt::Display;

    let mut buffer = File::create(c_source_path).unwrap();
    write!(
        buffer,
        r#"#include "husky_vm_interface.h"
"#
    );

    for ty in PRIMITIVE_TYPES {
        write!(buffer, "{}", CPrimitiveTypeRegistrationSource { ty })?
    }

    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        write!(buffer, "{}", CNonPrimitiveTypeRegistrationSource { ty })?
    }
    Ok(())
}
