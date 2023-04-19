[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`quick_sort::quick_sort`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`(independent v0: Type) -> Fp(core::slice::Slice v0) -> core::basic::unit`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`(independent v0: Type) -> Fp(core::slice::Slice v0, core::num::isize, core::num::isize) -> core::basic::unit`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`quick_sort::partition`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`(independent v0: Type) -> Fp(core::slice::Slice v0, core::num::isize, core::num::isize) -> core::num::isize`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
]