Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                Err(
                    Expr(
                        ExpectIdentifier(
                            TokenIdx(
                                22,
                            ),
                        ),
                    ),
                ),
            ),
            (
                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                Ok(
                    Form(
                        Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)