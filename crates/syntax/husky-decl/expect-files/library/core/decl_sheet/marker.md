Ok(
    DeclSheet {
        decls: [
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`core::marker::Copy`),
                        ast_idx: 0,
                        expr_region: ExprRegion(
                            Id {
                                value: 13,
                            },
                        ),
                        implicit_parameter_decl_list: None,
                    },
                ),
            ),
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`core::marker::Sized`),
                        ast_idx: 1,
                        expr_region: ExprRegion(
                            Id {
                                value: 14,
                            },
                        ),
                        implicit_parameter_decl_list: None,
                    },
                ),
            ),
        ],
    },
)