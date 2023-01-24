[
    Ok(
        Form(
            Feature(
                FeatureSignature {
                    output_ty: Application(
                        TermApplication {
                            function: Entity(
                                TypePath(`core::vec::Vec`, `Alien`),
                            ),
                            argument: Entity(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
    Ok(
        Form(
            Feature(
                FeatureSignature {
                    output_ty: Entity(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    Ok(
        Form(
            Feature(
                FeatureSignature {
                    output_ty: Entity(
                        TypePath(`core::num::f32`, `Alien`),
                    ),
                },
            ),
        ),
    ),
    Ok(
        Form(
            Feature(
                FeatureSignature {
                    output_ty: Application(
                        TermApplication {
                            function: Entity(
                                TypePath(`core::vec::Vec`, `Alien`),
                            ),
                            argument: Entity(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
    Ok(
        Form(
            Feature(
                FeatureSignature {
                    output_ty: Entity(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    Ok(
        Form(
            Feature(
                FeatureSignature {
                    output_ty: Entity(
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    ),
                },
            ),
        ),
    ),
    Ok(
        Form(
            Feature(
                FeatureSignature {
                    output_ty: Application(
                        TermApplication {
                            function: Entity(
                                TypePath(`core::vec::Vec`, `Alien`),
                            ),
                            argument: Entity(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
]