#include "husky_ml_datasets.h"
    
const __RegisterVTable __MNIST_LABEL_VTABLE = {
    .typename_str = "MnistLabel",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __mnist_label_clone,
    .drop = __mnist_label_drop,
    .eq = __mnist_label_eq,
    .assign = __mnist_label_assign,
};

const __RegisterVTable __BINARY_IMAGE_28_VTABLE = {
    .typename_str = "BinaryImage28",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __binary_image_28_clone,
    .drop = __binary_image_28_drop,
    .eq = __binary_image_28_eq,
    .assign = __binary_image_28_assign,
};

const __RegisterVTable __BINARY_GRID_28_VTABLE = {
    .typename_str = "BinaryGrid28",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __binary_grid_28_clone,
    .drop = __binary_grid_28_drop,
    .eq = __binary_grid_28_eq,
    .assign = __binary_grid_28_assign,
};

const __RegisterVTable __DATASET_VTABLE = {
    .typename_str = "Dataset",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __dataset_clone,
    .drop = __dataset_drop,
    .eq = __dataset_eq,
    .assign = __dataset_assign,
};
