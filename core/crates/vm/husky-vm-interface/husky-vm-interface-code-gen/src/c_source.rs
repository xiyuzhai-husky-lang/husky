use super::PRIMITIVE_TYPES;
use crate::NONPRIMITIVE_BUILTIN_TYPES;
use convert_case::{Case, Casing};
use std;
use std::fs::File;
use std::io::prelude::*;

pub fn write_c_source(c_source_path: &str) -> std::io::Result<()> {
    let mut buffer = File::create(c_source_path).unwrap();
    write!(
        buffer,
        r#"#include "husky_vm_interface.h"
"#
    );

    for ty in PRIMITIVE_TYPES {
        let uppercase_ty = ty.to_uppercase();
        write!(
            buffer,
            r#"
const __RegisterVTable __{uppercase_ty}_VTABLE = {{
    .typename_str = "{ty}",
    .primitive_value_to_bool = __{ty}_primitive_value_to_bool,
    .primitive_value_to_box = __{ty}_primitive_value_to_box,
    .drop = __{ty}_drop,
}};
"#
        )?
    }

    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        let lower_snake_ty = ty.to_case(Case::Snake);
        let upper_snake_ty = ty.to_case(Case::UpperSnake);
        write!(
            buffer,
            r#"
const __RegisterVTable __{upper_snake_ty}_VTABLE = {{
    .typename_str = "{ty}",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .drop = __{lower_snake_ty}_drop,
}};
"#
        )?
    }
    Ok(())
}
