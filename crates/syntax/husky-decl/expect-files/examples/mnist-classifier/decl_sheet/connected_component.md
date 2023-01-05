Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                Err(
                    Expr(
                        ExpectIdentifier(
                            TokenIdx(
                                33,
                            ),
                        ),
                    ),
                ),
            ),
            (
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                Err(
                    Expr(
                        ExpectIdentifier(
                            TokenIdx(
                                45,
                            ),
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
        ],
    },
)