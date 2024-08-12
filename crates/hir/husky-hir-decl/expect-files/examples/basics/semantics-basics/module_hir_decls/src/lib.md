```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Ritchie(
                MajorFunctionRitchieHirDecl {
                    path: MajorFormPath(`semantics_basics::some_neural_network`, `Ritchie(
                        Gn,
                    )`),
                    ritchie_item_kind: RitchieItemKind::Gn,
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirParenateParameters::Lazy(
                        HirLazyParenateParameters(
                            [
                                Simple {
                                    pattern_idx: 0,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::array::Array`, `Extern`),
                            template_arguments: [
                                HirTemplateArgument::Constant(
                                    HirConstant::USize(
                                        3,
                                    ),
                                ),
                                HirTemplateArgument::Type(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::f32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: false,
                        },
                    ),
                    hir_expr_region: Lazy(
                        HirLazyExprRegion(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]
```