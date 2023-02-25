[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort`, `Function`),
            ),
        ),
        Err(
            Derived(
                SignatureError,
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
            Derived(
                SignatureError,
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
            Term(`independent Type -> Fp(Slice $0, isize, isize) -> isize`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
            ),
        ),
        Err(
            Derived(
                SignatureError,
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
            Derived(
                SignatureError,
            ),
        ),
    ),
]