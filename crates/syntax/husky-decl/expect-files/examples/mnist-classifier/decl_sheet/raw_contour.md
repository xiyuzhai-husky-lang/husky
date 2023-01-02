Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                Err(
                    Expr(
                        ExpectIdentifier(
                            TokenIdx(
                                40,
                            ),
                        ),
                    ),
                ),
            ),
            (
                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                Ok(
                    Type(
                        Enum(
                            EnumTypeDecl {
                                path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                Err(
                    ExpectLCurlOrLParOrSemicolon(
                        TokenIdx(
                            887,
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)