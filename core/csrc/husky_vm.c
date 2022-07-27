#include "husky_vm.h"

#define nullptr 0
#define TODO nullptr

const __RegisterVTable __VOID_VTABLE = {
    .typename_str = "void",
    .primitive_value_to_bool = __void_primitive_value_to_bool,
    .primitive_value_to_box = __void_primitive_value_to_box,
    .drop = __void_drop,
};

// bool
bool __bool_primitive_value_to_bool(__RegisterData data) {
    return data.as_bool;
}
__X_primitive_value_to_box(bool);
const __RegisterVTable __BOOL_VTABLE = {
    .typename_str = "bool",
    .primitive_value_to_bool = __bool_primitive_value_to_bool,
    .primitive_value_to_box = __bool_primitive_value_to_box};

// i32
bool __i32_primitive_value_to_bool(__RegisterData data) { return data.as_i32; }
const __RegisterVTable __I32_VTABLE = {
    .typename_str = "i32",
    .primitive_value_to_bool = __i32_primitive_value_to_bool,
    .primitive_value_to_box = __i32_primitive_value_to_box};

// i64
const __RegisterVTable __I64_VTABLE = {
    .typename_str = "i64",
    .primitive_value_to_bool = __i64_primitive_value_to_bool,
    .primitive_value_to_box = __i64_primitive_value_to_box};

// b32
bool __b32_primitive_value_to_bool(__RegisterData data) { return data.as_b32; }
__X_primitive_value_to_box(b32);
const __RegisterVTable __B32_VTABLE = {.typename_str = "b32",
                                       .primitive_value_to_bool =
                                           __b32_primitive_value_to_bool,
                                       .primitive_value_to_box = nullptr};

// b64
bool __b64_primitive_value_to_bool(__RegisterData data) { return data.as_b64; }
__X_primitive_value_to_box(b64);
const __RegisterVTable __B64_VTABLE = {.typename_str = "b64",
                                       .primitive_value_to_bool =
                                           __b64_primitive_value_to_bool,
                                       .primitive_value_to_box = nullptr};

// f32
bool __f32_primitive_value_to_bool(__RegisterData data) { return data.as_f32; }
__X_primitive_value_to_box(f32);
const __RegisterVTable __F32_VTABLE = {.typename_str = "f32",
                                       .primitive_value_to_bool =
                                           __f32_primitive_value_to_bool,
                                       .primitive_value_to_box = nullptr,
                                       .drop = nullptr};

// f64
bool __f64_primitive_value_to_bool(__RegisterData data) { return data.as_f64; }
__X_primitive_value_to_box(f64);
const __RegisterVTable __F64_VTABLE = {.typename_str = "f64",
                                       .primitive_value_to_bool =
                                           __f64_primitive_value_to_bool,
                                       .primitive_value_to_box = nullptr};

const __RegisterVTable __BINARY_GRID28_VTABLE = {
    .typename_str = "domains::ml::datasets::cv::mnist::BinaryGrid28",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr};

// ad hoc
// domains::ml::datasets::cv::mnist::BinaryImage2
const __RegisterVTable __BINARY_IMAGE28_VTABLE = {
    .typename_str = "domains::ml::datasets::cv::mnist::BinaryImage28",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr};

// Dataset
const __RegisterVTable __DATASET_VTABLE = {.typename_str = "Dataset",
                                           .primitive_value_to_bool = nullptr,
                                           .primitive_value_to_box = nullptr};
const __RegisterVTable __VIRTUAL_VEC_VTABLE = {
    .typename_str = "VirtualVec",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr};
const __RegisterVTable __VIRTUAL_CYCLIC_SLICE_VTABLE = {
    .typename_str = "VirtualCyclicSlice",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr};
const __RegisterVTable __VIRTUAL_STRUCT_VTABLE = {
    .typename_str = "VirtualStruct",
    .primitive_value_to_bool = nullptr,
    .primitive_value_to_box = nullptr};