Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::basic::bool`, `Foreign`),
                            ast_idx: 0,
                            expr_page: ExprPage(
                                Id {
                                    value: 1,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`core::basic::Trait`, `Structure`),
                            ast_idx: 1,
                            expr_page: ExprPage(
                                Id {
                                    value: 2,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`core::basic::Module`, `Structure`),
                            ast_idx: 2,
                            expr_page: ExprPage(
                                Id {
                                    value: 3,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
        ],
    },
)