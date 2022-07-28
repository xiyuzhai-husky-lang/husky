#include "husky_vm_interface.h"

const __RegisterVTable __VOID_VTABLE = {
    .typename_str = "void",
    .primitive_value_to_bool = __void_primitive_value_to_bool,
    .primitive_value_to_box = __void_primitive_value_to_box,
    .drop = __void_drop,
};

const __RegisterVTable __BOOL_VTABLE = {
    .typename_str = "bool",
    .primitive_value_to_bool = __bool_primitive_value_to_bool,
    .primitive_value_to_box = __bool_primitive_value_to_box,
    .drop = __bool_drop,
};

const __RegisterVTable __I32_VTABLE = {
    .typename_str = "i32",
    .primitive_value_to_bool = __i32_primitive_value_to_bool,
    .primitive_value_to_box = __i32_primitive_value_to_box,
    .drop = __i32_drop,
};

const __RegisterVTable __I64_VTABLE = {
    .typename_str = "i64",
    .primitive_value_to_bool = __i64_primitive_value_to_bool,
    .primitive_value_to_box = __i64_primitive_value_to_box,
    .drop = __i64_drop,
};

const __RegisterVTable __B32_VTABLE = {
    .typename_str = "b32",
    .primitive_value_to_bool = __b32_primitive_value_to_bool,
    .primitive_value_to_box = __b32_primitive_value_to_box,
    .drop = __b32_drop,
};

const __RegisterVTable __B64_VTABLE = {
    .typename_str = "b64",
    .primitive_value_to_bool = __b64_primitive_value_to_bool,
    .primitive_value_to_box = __b64_primitive_value_to_box,
    .drop = __b64_drop,
};

const __RegisterVTable __F32_VTABLE = {
    .typename_str = "f32",
    .primitive_value_to_bool = __f32_primitive_value_to_bool,
    .primitive_value_to_box = __f32_primitive_value_to_box,
    .drop = __f32_drop,
};

const __RegisterVTable __F64_VTABLE = {
    .typename_str = "f64",
    .primitive_value_to_bool = __f64_primitive_value_to_bool,
    .primitive_value_to_box = __f64_primitive_value_to_box,
    .drop = __f64_drop,
};
