```rust
[
    (
        ItemPath(`core::task::IsTask`),
        Ok(
            ItemDecTemplate::MajorItem(
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
        ItemPath(`core::task::Task`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::TypeVar(
                        TypeVarDecTemplate {
                            default: None,
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::task::TASK`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::StaticVar(
                        MajorStaticVarDecTemplate {
                            return_ty: DecTerm::ItemPath(
                                DecItemPath::Form(
                                    MajorFormPath(`core::task::Task`, `TypeVar`),
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