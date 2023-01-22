Ok(
    DefnSheet {
        defns: [
            Type(
                RegularStruct(
                    RegularStructTypeDefn {
                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        decl: RegularStructTypeDecl(
                            Id {
                                value: 14,
                            },
                        ),
                    },
                ),
            ),
            Type(
                Enum(
                    EnumTypeDefn {
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        decl: EnumTypeDecl(
                            Id {
                                value: 1,
                            },
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 33,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 317,
                            },
                        ),
                        body: Ok(
                            9,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 34,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 318,
                            },
                        ),
                        body: Ok(
                            6,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 35,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 319,
                            },
                        ),
                        body: Ok(
                            9,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 36,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 320,
                            },
                        ),
                        body: Ok(
                            8,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 37,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 321,
                            },
                        ),
                        body: Ok(
                            15,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 38,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 322,
                            },
                        ),
                        body: Ok(
                            8,
                        ),
                    },
                ),
            ),
            Type(
                RegularStruct(
                    RegularStructTypeDefn {
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                        decl: RegularStructTypeDecl(
                            Id {
                                value: 15,
                            },
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 39,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 323,
                            },
                        ),
                        body: Ok(
                            30,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                        decl: FunctionDecl(
                            Id {
                                value: 40,
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 324,
                            },
                        ),
                        body: Ok(
                            214,
                        ),
                    },
                ),
            ),
            ImplBlock(
                TypeImplBlock(
                    TypeImplBlockDecl {
                        ast_idx: 200,
                        impl_block: ImplBlock(
                            Id {
                                value: 18,
                            },
                        ),
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                42,
                            ),
                        },
                        implicit_parameter_decl_list: None,
                        ty: TypeExpr {
                            expr: 0,
                        },
                        eol_colon: Ok(
                            EolColonToken {
                                token_idx: TokenIdx(
                                    44,
                                ),
                            },
                        ),
                        expr_region: ExprRegion(
                            Id {
                                value: 311,
                            },
                        ),
                    },
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Memo(
                        TypeMemoDefn {
                            path: Some(
                                TypeItemPath(
                                    Id {
                                        value: 64,
                                    },
                                ),
                            ),
                            decl: TypeMemoDecl(
                                Id {
                                    value: 20,
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 325,
                                },
                            ),
                            body: Ok(
                                4,
                            ),
                        },
                    ),
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Memo(
                        TypeMemoDefn {
                            path: Some(
                                TypeItemPath(
                                    Id {
                                        value: 65,
                                    },
                                ),
                            ),
                            decl: TypeMemoDecl(
                                Id {
                                    value: 21,
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 326,
                                },
                            ),
                            body: Ok(
                                55,
                            ),
                        },
                    ),
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Memo(
                        TypeMemoDefn {
                            path: Some(
                                TypeItemPath(
                                    Id {
                                        value: 66,
                                    },
                                ),
                            ),
                            decl: TypeMemoDecl(
                                Id {
                                    value: 22,
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 327,
                                },
                            ),
                            body: Ok(
                                9,
                            ),
                        },
                    ),
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Memo(
                        TypeMemoDefn {
                            path: Some(
                                TypeItemPath(
                                    Id {
                                        value: 67,
                                    },
                                ),
                            ),
                            decl: TypeMemoDecl(
                                Id {
                                    value: 23,
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 328,
                                },
                            ),
                            body: Ok(
                                65,
                            ),
                        },
                    ),
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Method(
                        TypeMethodDefn {
                            path: Some(
                                TypeItemPath(
                                    Id {
                                        value: 68,
                                    },
                                ),
                            ),
                            decl: TypeMethodDecl(
                                Id {
                                    value: 45,
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 329,
                                },
                            ),
                            body: Ok(
                                18,
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)