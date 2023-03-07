[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort`, `Function`),
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
                FormPath(`quick_sort::quick_sort_aux`, `Function`),
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
                FormPath(`quick_sort::partition`, `Function`),
            ),
        ),
        Ok(
            RawTerm(`(independent t: Type) -> Fp(core::slice::Slice t, core::num::isize, core::num::isize) -> core::num::isize`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
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
                FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
            ),
        ),
        Err(
            RawTypeError::Derived(
                DerivedRawTypeError::SignatureError,
            ),
        ),
    ),
]