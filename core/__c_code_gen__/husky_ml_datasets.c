#include "husky_ml_datasets.h"
    
const __RegisterVTable __BINARY_IMAGE_28_VTABLE = {
    .typename_str = "BinaryImage28",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .drop = __binary_image_28_drop,
};

const __RegisterVTable __BINARY_GRID_28_VTABLE = {
    .typename_str = "BinaryGrid28",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .drop = __binary_grid_28_drop,
};

const __RegisterVTable __DATASET_VTABLE = {
    .typename_str = "Dataset",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .drop = __dataset_drop,
};
