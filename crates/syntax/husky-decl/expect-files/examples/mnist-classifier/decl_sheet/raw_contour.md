Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    PropsStruct(
                        PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Enum(
                        EnumTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                        },
                    ),
                ),
            ),
            Err(
                ExpectLCurlOrLParOrSemicolon(
                    TokenIdx(
                        889,
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl,
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 46,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 48,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 49,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 50,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)