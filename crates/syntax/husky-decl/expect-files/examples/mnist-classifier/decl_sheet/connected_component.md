Ok(
    DeclSheet {
        decls: [
            Err(
                Expr(
                    ExpectIdentifier(
                        TokenIdx(
                            33,
                        ),
                    ),
                ),
            ),
            Err(
                Expr(
                    ExpectIdentifier(
                        TokenIdx(
                            45,
                        ),
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    PropsStruct(
                        PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                        },
                    ),
                ),
            ),
        ],
    },
)