#include "husky_vm_interface.h"

const __RegisterVTable __VOID_VTABLE = {
    .typename_str = "void",
    .primitive_value_to_bool = __void_primitive_value_to_bool,
    .primitive_value_to_box = __void_primitive_value_to_box,
    .clone = __void_clone,
    .drop = __void_drop,
    .eq = __void_eq,
    .assign = __void_assign,
};

const __RegisterVTable __BOOL_VTABLE = {
    .typename_str = "bool",
    .primitive_value_to_bool = __bool_primitive_value_to_bool,
    .primitive_value_to_box = __bool_primitive_value_to_box,
    .clone = __bool_clone,
    .drop = __bool_drop,
    .eq = __bool_eq,
    .assign = __bool_assign,
};

const __RegisterVTable __I32_VTABLE = {
    .typename_str = "i32",
    .primitive_value_to_bool = __i32_primitive_value_to_bool,
    .primitive_value_to_box = __i32_primitive_value_to_box,
    .clone = __i32_clone,
    .drop = __i32_drop,
    .eq = __i32_eq,
    .assign = __i32_assign,
};

const __RegisterVTable __I64_VTABLE = {
    .typename_str = "i64",
    .primitive_value_to_bool = __i64_primitive_value_to_bool,
    .primitive_value_to_box = __i64_primitive_value_to_box,
    .clone = __i64_clone,
    .drop = __i64_drop,
    .eq = __i64_eq,
    .assign = __i64_assign,
};

const __RegisterVTable __B32_VTABLE = {
    .typename_str = "b32",
    .primitive_value_to_bool = __b32_primitive_value_to_bool,
    .primitive_value_to_box = __b32_primitive_value_to_box,
    .clone = __b32_clone,
    .drop = __b32_drop,
    .eq = __b32_eq,
    .assign = __b32_assign,
};

const __RegisterVTable __B64_VTABLE = {
    .typename_str = "b64",
    .primitive_value_to_bool = __b64_primitive_value_to_bool,
    .primitive_value_to_box = __b64_primitive_value_to_box,
    .clone = __b64_clone,
    .drop = __b64_drop,
    .eq = __b64_eq,
    .assign = __b64_assign,
};

const __RegisterVTable __F32_VTABLE = {
    .typename_str = "f32",
    .primitive_value_to_bool = __f32_primitive_value_to_bool,
    .primitive_value_to_box = __f32_primitive_value_to_box,
    .clone = __f32_clone,
    .drop = __f32_drop,
    .eq = __f32_eq,
    .assign = __f32_assign,
};

const __RegisterVTable __F64_VTABLE = {
    .typename_str = "f64",
    .primitive_value_to_bool = __f64_primitive_value_to_bool,
    .primitive_value_to_box = __f64_primitive_value_to_box,
    .clone = __f64_clone,
    .drop = __f64_drop,
    .eq = __f64_eq,
    .assign = __f64_assign,
};

const __RegisterVTable __VIRTUAL_FUNCTION_VTABLE = {
    .typename_str = "__VirtualFunction",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __virtual_function_clone,
    .drop = __virtual_function_drop,
    .eq = __virtual_function_eq,
    .assign = __virtual_function_assign,
};

const __RegisterVTable __VIRTUAL_ENUM_VTABLE = {
    .typename_str = "__VirtualEnum",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __virtual_enum_clone,
    .drop = __virtual_enum_drop,
    .eq = __virtual_enum_eq,
    .assign = __virtual_enum_assign,
};
