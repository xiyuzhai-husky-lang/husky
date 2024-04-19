```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Ritchie(
                MajorRitchieHirDecl {
                    path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                        Fn,
                    )`),
                    ritchie_item_kind: RitchieItemKind::Fn,
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirParenateParameters::Eager(
                        HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_expr_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Simple {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::basic::bool`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 128,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]
```