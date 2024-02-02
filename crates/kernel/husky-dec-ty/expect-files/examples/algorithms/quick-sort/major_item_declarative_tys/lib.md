[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`(independent v0: Type) -> fn((core::slice::Slice v0) -> core::basic::unit`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`(independent v0: Type) -> fn((core::slice::Slice v0, core::num::isize, core::num::isize) -> core::basic::unit`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::partition`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`(independent v0: Type) -> fn((core::slice::Slice v0, core::num::isize, core::num::isize) -> core::num::isize`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn(() -> core::basic::unit`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTerm(`fn(() -> core::basic::unit`),
        ),
    ),
]