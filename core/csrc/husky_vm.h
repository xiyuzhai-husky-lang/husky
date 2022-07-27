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

typedef bool (*primitive_value_to_bool_t)(__RegisterData);

typedef struct __RegisterVTable {
    char const *typename;
    primitive_value_to_bool_t primitive_value_to_bool;
} __RegisterVTable;
extern const __RegisterVTable __VOID_VTABLE;
extern const __RegisterVTable __BOOL_VTABLE;
extern const __RegisterVTable __I32_VTABLE;
extern const __RegisterVTable __I64_VTABLE;
extern const __RegisterVTable __B32_VTABLE;
extern const __RegisterVTable __B64_VTABLE;
extern const __RegisterVTable __F32_VTABLE;
extern const __RegisterVTable __F64_VTABLE;

// ad hoc
extern const __RegisterVTable __BINARY_IMAGE28_VTABLE;
extern const __RegisterVTable __BINARY_GRID28_VTABLE;
extern const __RegisterVTable __DATASET_VTABLE;
extern const __RegisterVTable __VIRTUAL_VEC_VTABLE;
extern const __RegisterVTable __VIRTUAL_CYCLIC_SLICE_VTABLE;
extern const __RegisterVTable __VIRTUAL_STRUCT_VTABLE;