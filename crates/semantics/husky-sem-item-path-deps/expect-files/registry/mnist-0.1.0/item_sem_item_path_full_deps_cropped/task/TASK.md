```rust
Some(
    [
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist::TASK`, `StaticVar`),
            ),
        ),
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist::task::MnistTask`, `Extern`),
            ),
        ),
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist::task::MnistTask(0)::new`,
                    TypeItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
    ],
)
```