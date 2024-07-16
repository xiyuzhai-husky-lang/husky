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
                                DecTerm::EntityPath(
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
                            return_ty: DecTerm::EntityPath(
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