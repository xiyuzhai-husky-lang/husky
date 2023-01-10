Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    PropsStruct(
                        PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    PropsStruct(
                        PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                        },
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
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl,
                    ),
                ),
            ),
        ],
    },
)