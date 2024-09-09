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
                                DecTerm::ItemPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist::task::MnistTask`, `Extern`),
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
        ItemPath(`mnist::TASK`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::StaticVar(
                        MajorStaticVarDecTemplate {
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Type(
                                    TypePath(`mnist::task::MnistTask`, `Extern`),
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