```rust
[
    (
        ItemPath(`mnist::Task`),
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
        ItemPath(`mnist::task`),
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