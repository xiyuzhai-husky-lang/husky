```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::task::Task`, `Extern`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternTypeDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`core::task::TASK`, `Static`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Static(
                        MajorStaticDecTemplate {
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 84,
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