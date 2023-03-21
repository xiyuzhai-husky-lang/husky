[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort`, `Fn`),
            ),
        ),
        Err(
            RawTypeError::Derived(
                DerivedRawTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort_aux`, `Fn`),
            ),
        ),
        Err(
            RawTypeError::Derived(
                DerivedRawTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::partition`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`(independent t: Type) -> Fp(core::slice::Slice t, core::num::isize, core::num::isize) -> core::num::isize`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
            ),
        ),
        Err(
            RawTypeError::Derived(
                DerivedRawTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
            ),
        ),
        Err(
            RawTypeError::Derived(
                DerivedRawTypeError::SignatureError,
            ),
        ),
    ),
]