```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist::Task`, `TypeVar`),
            ),
        ),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::TypeVar(
                        TypeVarDecTemplate {
                            default: Some(
                                EntityPath(
                                    Type(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist::TASK`, `StaticVar`),
            ),
        ),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::StaticVar(
                        MajorStaticVarDecTemplate {
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 16,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```