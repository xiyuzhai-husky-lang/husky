[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`(independent v0: Type) -> fn(core::slice::Slice v0) -> core::basic::unit`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`(independent v0: Type) -> fn(core::slice::Slice v0, core::num::isize, core::num::isize) -> core::basic::unit`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::partition`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`(independent v0: Type) -> fn(core::slice::Slice v0, core::num::isize, core::num::isize) -> core::num::isize`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn() -> core::basic::unit`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`fn() -> core::basic::unit`),
        ),
    ),
]