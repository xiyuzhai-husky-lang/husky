Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                Err(
                    ExpectLCurlOrLParOrSemicolon(
                        TokenIdx(
                            30,
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)