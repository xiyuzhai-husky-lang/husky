#include "husky_any.h"
    
const __RegisterVTable __VIRTUAL_STRUCT_VTABLE = {
    .typename_str = "VirtualStruct",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __virtual_struct_clone,
    .drop = __virtual_struct_drop,
    .eq = __virtual_struct_eq,
};

const __RegisterVTable __VIRTUAL_VEC_VTABLE = {
    .typename_str = "VirtualVec",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __virtual_vec_clone,
    .drop = __virtual_vec_drop,
    .eq = __virtual_vec_eq,
};

const __RegisterVTable __VIRTUAL_CYCLIC_SLICE_VTABLE = {
    .typename_str = "VirtualCyclicSlice",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __virtual_cyclic_slice_clone,
    .drop = __virtual_cyclic_slice_drop,
    .eq = __virtual_cyclic_slice_eq,
};
