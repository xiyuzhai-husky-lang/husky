[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Fn(
                        MajorFnDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DecTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DecTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm(`core::basic::bool`),
                        },
                    ),
                ),
            ),
        ),
    ),
]