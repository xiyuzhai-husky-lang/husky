#include "husky_vm.h"

const __RegisterVTable __VOID_VTABLE = {typename : "void"};
const __RegisterVTable __BOOL_VTABLE = {typename : "bool"};
const __RegisterVTable __I32_VTABLE = {typename : "i32"};
const __RegisterVTable __I64_VTABLE = {typename : "i64"};
const __RegisterVTable __B32_VTABLE = {typename : "b32"};
const __RegisterVTable __B64_VTABLE = {typename : "b64"};
const __RegisterVTable __F32_VTABLE = {typename : "f32"};
const __RegisterVTable __F64_VTABLE = {typename : "f64"};

// ad hoc
const __RegisterVTable __BINARY_IMAGE28_VTABLE = {typename : "f64"};
const __RegisterVTable __BINARY_GRID28_VTABLE = {typename : "f64"};
const __RegisterVTable __DATASET_VTABLE = {typename : "Dataset"};
const __RegisterVTable __VIRTUAL_VEC_VTABLE = {typename : "VirtualVec"};
const __RegisterVTable __VIRTUAL_CYCLIC_SLICE_VTABLE = {
    typename : "VirtualCyclicSlice"
};
const __RegisterVTable __VIRTUAL_STRUCT_VTABLE = {typename : "VirtualStruct"};