use super::PRIMITIVE_TYPES;
use crate::NONPRIMITIVE_BUILTIN_TYPES;
use convert_case::{Case, Casing};
use std;
use std::fs::File;
use std::io::prelude::*;

pub fn write_header(c_header_path: &str) -> std::io::Result<()> {
    let mut buffer = File::create(c_header_path).unwrap();
    write!(
        buffer,
        r#"#pragma once
#include <stdbool.h>
#include <stdint.h>

typedef struct unit {{
}} unit;
typedef union __RegisterData {{
    unit as_void;
    bool as_bool;
    int32_t as_i32;
    int64_t as_i64;
    uint32_t as_b32;
    uint64_t as_b64;
    float as_f32;
    double as_f64;
    void *as_opt_ptr;
}} __RegisterData;

typedef bool (*__primitive_value_to_bool_t)(__RegisterData);

typedef void *(*__primitive_value_to_box_t)(__RegisterData *);

typedef void (*__drop_t)(void *);

typedef struct __RegisterVTable {{
    char const *typename_str;
    __primitive_value_to_bool_t primitive_value_to_bool;
    __primitive_value_to_box_t primitive_value_to_box;
    __drop_t drop;
}} __RegisterVTable;
    
// handles of primitive types are provided by Rust
"#
    )?;
    for ty in PRIMITIVE_TYPES {
        let uppercase_ty = ty.to_uppercase();
        write!(
            buffer,
            r#"
// {ty}
extern bool __{ty}_primitive_value_to_bool(__RegisterData data);
extern void *__{ty}_primitive_value_to_box(__RegisterData *data);
extern void __{ty}_drop(void*);
extern const __RegisterVTable __{uppercase_ty}_VTABLE;
        "#
        )?
    }
    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        let lower_snake_ty = ty.to_case(Case::Snake);
        let upper_snake_ty = ty.to_case(Case::UpperSnake);
        write!(
            buffer,
            r#"
// {ty}
extern void __{lower_snake_ty}_drop(void*);
extern const __RegisterVTable __{upper_snake_ty}_VTABLE;
        "#
        )?
    }
    Ok(())
}
