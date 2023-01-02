Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                Err(
                    Expr(
                        ExpectRightCurlyBrace(
                            TokenIdx(
                                42,
                            ),
                        ),
                    ),
                ),
            ),
            (
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                Err(
                    Expr(
                        ExpectRightCurlyBrace(
                            TokenIdx(
                                153,
                            ),
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)