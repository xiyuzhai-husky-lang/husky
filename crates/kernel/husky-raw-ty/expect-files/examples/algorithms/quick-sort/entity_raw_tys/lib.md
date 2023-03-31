[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`(independent v0: Type) -> Fp(core::slice::Slice v0) -> core::basic::unit`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort_aux`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`(independent v0: Type) -> Fp(core::slice::Slice v0, core::num::isize, core::num::isize) -> core::basic::unit`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::partition`, `Fn`),
            ),
        ),
        Ok(
            RawTerm(`(independent v0: Type) -> Fp(core::slice::Slice v0, core::num::isize, core::num::isize) -> core::num::isize`),
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