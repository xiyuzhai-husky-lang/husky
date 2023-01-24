Ok(
    DeclSheet {
        decls: [
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`core::cmp::PartialEq`),
                        ast_idx: 0,
                        expr_region: ExprRegion(
                            Id {
                                value: 5,
                            },
                        ),
                        implicit_parameter_decl_list: None,
                    },
                ),
            ),
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`core::cmp::Eq`),
                        ast_idx: 1,
                        expr_region: ExprRegion(
                            Id {
                                value: 6,
                            },
                        ),
                        implicit_parameter_decl_list: None,
                    },
                ),
            ),
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`core::cmp::PartialOrd`),
                        ast_idx: 2,
                        expr_region: ExprRegion(
                            Id {
                                value: 7,
                            },
                        ),
                        implicit_parameter_decl_list: None,
                    },
                ),
            ),
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`core::cmp::Ord`),
                        ast_idx: 3,
                        expr_region: ExprRegion(
                            Id {
                                value: 8,
                            },
                        ),
                        implicit_parameter_decl_list: None,
                    },
                ),
            ),
        ],
    },
)