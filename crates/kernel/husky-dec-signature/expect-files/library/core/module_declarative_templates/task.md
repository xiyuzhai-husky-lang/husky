```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::task::IsTask`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Trait(
                    TraitDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSymbolicVariable(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                    },
                ),
            ),
        ),
    ),
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
                                                value: 94,
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