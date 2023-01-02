Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                Err(
                    Expr(
                        ExpectRightCurlyBrace(
                            TokenIdx(
                                21,
                            ),
                        ),
                    ),
                ),
            ),
            (
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                Ok(
                    Type(
                        PropsStruct(
                            PropsStructTypeDecl {
                                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                Err(
                    ExpectLCurlOrLParOrSemicolon(
                        TokenIdx(
                            73,
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)