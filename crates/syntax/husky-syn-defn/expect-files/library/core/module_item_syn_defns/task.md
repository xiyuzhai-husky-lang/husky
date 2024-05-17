```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::task::IsTask`),
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TraitItem(
                TraitItemPath(`core::task::IsTask::Backend`, `AssocType`),
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TraitItem(
                TraitItemPath(`core::task::IsTask::Frontend`, `AssocType`),
            ),
        ),
        None,
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::task::Task`, `Extern`),
            ),
        ),
        None,
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`core::task::TASK`, `Static`),
            ),
        ),
        None,
    ),
]
```