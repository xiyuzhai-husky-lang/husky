#include "husky_vm.h"

#define nullptr 0
#define TODO nullptr

const __RegisterVTable __VOID_VTABLE = {
    typename : "void",
    primitive_value_to_bool : nullptr
};

// bool
bool __bool_primitive_value_to_bool(__RegisterData data) {
    return data.as_bool;
}
const __RegisterVTable __BOOL_VTABLE = {
    typename : "bool",
    primitive_value_to_bool : __bool_primitive_value_to_bool
};

// i32
bool __i32_primitive_value_to_bool(__RegisterData data) { return data.as_i32; }
const __RegisterVTable __I32_VTABLE = {
    typename : "i32",
    primitive_value_to_bool : __i32_primitive_value_to_bool
};

// i64
bool __i64_primitive_value_to_bool(__RegisterData data) { return data.as_i64; }
const __RegisterVTable __I64_VTABLE = {
    typename : "i64",
    primitive_value_to_bool : __i64_primitive_value_to_bool
};

// b32
bool __b32_primitive_value_to_bool(__RegisterData data) { return data.as_b32; }
const __RegisterVTable __B32_VTABLE = {
    typename : "b32",
    primitive_value_to_bool : __b32_primitive_value_to_bool
};

// b64
bool __b64_primitive_value_to_bool(__RegisterData data) { return data.as_b64; }
const __RegisterVTable __B64_VTABLE = {
    typename : "b64",
    primitive_value_to_bool : __b64_primitive_value_to_bool
};

// f32
bool __f32_primitive_value_to_bool(__RegisterData data) { return data.as_f32; }
const __RegisterVTable __F32_VTABLE = {
    typename : "f32",
    primitive_value_to_bool : __f32_primitive_value_to_bool
};

// f64
bool __f64_primitive_value_to_bool(__RegisterData data) { return data.as_f64; }
const __RegisterVTable __F64_VTABLE = {
    typename : "f64",
    primitive_value_to_bool : __f64_primitive_value_to_bool
};

const __RegisterVTable __BINARY_GRID28_VTABLE = {
    typename : "domains::ml::datasets::cv::mnist::BinaryGrid28",
    primitive_value_to_bool : nullptr
};

// ad hoc
// domains::ml::datasets::cv::mnist::BinaryImage2
const __RegisterVTable __BINARY_IMAGE28_VTABLE = {
    typename : "domains::ml::datasets::cv::mnist::BinaryImage28",
    primitive_value_to_bool : nullptr
};

// Dataset
const __RegisterVTable __DATASET_VTABLE = {
    typename : "Dataset",
    primitive_value_to_bool : nullptr
};
const __RegisterVTable __VIRTUAL_VEC_VTABLE = {
    typename : "VirtualVec",
    primitive_value_to_bool : nullptr
};
const __RegisterVTable __VIRTUAL_CYCLIC_SLICE_VTABLE = {
    typename : "VirtualCyclicSlice",
    primitive_value_to_bool : nullptr
};
const __RegisterVTable __VIRTUAL_STRUCT_VTABLE = {
    typename : "VirtualStruct",
    primitive_value_to_bool : nullptr
};