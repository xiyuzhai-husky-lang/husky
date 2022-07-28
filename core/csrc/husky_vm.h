#pragma once
#include <stdbool.h>
#include <stdint.h>

typedef struct unit {
} unit;
typedef union __RegisterData {
    unit as_void;
    bool as_bool;
    int32_t as_i32;
    int64_t as_i64;
    uint32_t as_b32;
    uint64_t as_b64;
    float as_f32;
    double as_f64;
    void *as_opt_ptr;
} __RegisterData;

typedef bool (*__primitive_value_to_bool_t)(__RegisterData);

typedef void *(*__primitive_value_to_box_t)(__RegisterData *);

typedef void (*__drop_t)(void *);

typedef struct __RegisterVTable {
    char const *typename_str;
    __primitive_value_to_bool_t primitive_value_to_bool;
    __primitive_value_to_box_t primitive_value_to_box;
    __drop_t drop;
} __RegisterVTable;

// handles of primitive types are provided by Rust
// void type
extern bool __void_primitive_value_to_bool(__RegisterData data);
extern void *__void_primitive_value_to_box(__RegisterData *data);
extern void __void_drop(void *);
extern const __RegisterVTable __VOID_VTABLE;
extern const __RegisterVTable __BOOL_VTABLE;
extern const __RegisterVTable __I32_VTABLE;
extern const __RegisterVTable __I64_VTABLE;
extern const __RegisterVTable __B32_VTABLE;
extern const __RegisterVTable __B64_VTABLE;
extern const __RegisterVTable __F32_VTABLE;
extern const __RegisterVTable __F64_VTABLE;

// ad hoc
extern const __RegisterVTable __BINARY_IMAGE_28_VTABLE;
extern const __RegisterVTable __BINARY_GRID_28_VTABLE;
extern const __RegisterVTable __DATASET_VTABLE;
extern const __RegisterVTable __VIRTUAL_VEC_VTABLE;
extern const __RegisterVTable __VIRTUAL_CYCLIC_SLICE_VTABLE;
extern const __RegisterVTable __VIRTUAL_STRUCT_VTABLE;