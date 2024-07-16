```rust
[
    (
        ItemPath(`mnist_classifier::digits::five::is_five`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                            argument: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                ),
                                            ),
                                        },
                                    ),
                                    argument: DecTerm::EntityPath(
                                        DecItemPath::TypeVariant(
                                            TypeVariantPath(`mnist::MnistLabel::Five`),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```