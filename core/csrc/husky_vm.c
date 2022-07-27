#include "husky_vm.h"
#include <stdlib.h>

#define nullptr 0
#define TODO nullptr
#define i32 int32_t
#define i64 int64_t
#define b32 uint32_t
#define b64 uint64_t
#define f32 float
#define f64 double
#define __X_primitive_value_to_box(x)                                          \
    void *__##x##_primitive_value_to_box(__RegisterData *data) {               \
        x *ptr = (x *)malloc(sizeof(x));                                       \
        *ptr = data->as_##x;                                                   \
        return ptr;                                                            \
    }

const __RegisterVTable __VOID_VTABLE = {
    .typename =  "void",
    .primitive_value_to_bool =  nullptr,
    .primitive_value_to_box =  nullptr
};

// bool
bool __bool_primitive_value_to_bool(__RegisterData data) {
    return data.as_bool;
}
__X_primitive_value_to_box(bool);
const __RegisterVTable __BOOL_VTABLE = {
    .typename =  "bool",
    .primitive_value_to_bool =  __bool_primitive_value_to_bool,
    .primitive_value_to_box =  __bool_primitive_value_to_box
};

// i32
bool __i32_primitive_value_to_bool(__RegisterData data) { return data.as_i32; }
__X_primitive_value_to_box(i32);
const __RegisterVTable __I32_VTABLE = {
    .typename =  "i32",
    .primitive_value_to_bool =  __i32_primitive_value_to_bool,
    .primitive_value_to_box =  __i32_primitive_value_to_box
};

// i64
bool __i64_primitive_value_to_bool(__RegisterData data) { return data.as_i64; }
__X_primitive_value_to_box(i64);
const __RegisterVTable __I64_VTABLE = {
    .typename =  "i64",
    .primitive_value_to_bool =  __i64_primitive_value_to_bool,
    .primitive_value_to_box =  __i64_primitive_value_to_box
};

// b32
bool __b32_primitive_value_to_bool(__RegisterData data) { return data.as_b32; }
__X_primitive_value_to_box(b32);
const __RegisterVTable __B32_VTABLE = {
    .typename =  "b32",
    .primitive_value_to_bool =  __b32_primitive_value_to_bool,
    .primitive_value_to_box =  nullptr
};

// b64
bool __b64_primitive_value_to_bool(__RegisterData data) { return data.as_b64; }
__X_primitive_value_to_box(b64);
const __RegisterVTable __B64_VTABLE = {
    .typename =  "b64",
    .primitive_value_to_bool =  __b64_primitive_value_to_bool,
    .primitive_value_to_box =  nullptr
};

// f32
bool __f32_primitive_value_to_bool(__RegisterData data) { return data.as_f32; }
__X_primitive_value_to_box(f32);
const __RegisterVTable __F32_VTABLE = {
    .typename =  "f32",
    .primitive_value_to_bool =  __f32_primitive_value_to_bool,
    .primitive_value_to_box =  nullptr
};

// f64
bool __f64_primitive_value_to_bool(__RegisterData data) { return data.as_f64; }
__X_primitive_value_to_box(f64);
const __RegisterVTable __F64_VTABLE = {
    .typename = "f64",
    .primitive_value_to_bool = __f64_primitive_value_to_bool,
    .primitive_value_to_box = nullptr
};

const __RegisterVTable __BINARY_GRID28_VTABLE = {
   . typename = "domains::ml::datasets::cv::mnist::BinaryGrid28",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr
};

// ad hoc
// domains::ml::datasets::cv::mnist::BinaryImage2
const __RegisterVTable __BINARY_IMAGE28_VTABLE = {
    .typename = "domains::ml::datasets::cv::mnist::BinaryImage28",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr
};

// Dataset
const __RegisterVTable __DATASET_VTABLE = {
    .typename = "Dataset",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr
};
const __RegisterVTable __VIRTUAL_VEC_VTABLE = {
    .typename = "VirtualVec",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr
};
const __RegisterVTable __VIRTUAL_CYCLIC_SLICE_VTABLE = {
    .typename = "VirtualCyclicSlice",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr
};
const __RegisterVTable __VIRTUAL_STRUCT_VTABLE = {
    .typename = "VirtualStruct",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr
};