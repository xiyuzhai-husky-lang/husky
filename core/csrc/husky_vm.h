
typedef struct __RegisterVTable {
    char const *typename;
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