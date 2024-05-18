```rust
[
    ItemPath::MajorItem(
        MajorItemPath::Trait(
            TraitPath(`core::task::IsTask`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitItem(
            TraitItemPath(`core::task::IsTask::Backend`, `AssocType`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TraitItem(
            TraitItemPath(`core::task::IsTask::Frontend`, `AssocType`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`core::task::Task`, `Extern`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Form(
            FormPath(`core::task::TASK`, `Static`),
        ),
    ),
]
```