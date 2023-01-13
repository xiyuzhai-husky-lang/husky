Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Foreign(
                        AlienTypeDecl {
                            path: TypePath(`core::basic::bool`, `Foreign`),
                            ast_idx: 0,
                            expr_sheet: ExprSheet(
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
                            expr_sheet: ExprSheet(
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
                            expr_sheet: ExprSheet(
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