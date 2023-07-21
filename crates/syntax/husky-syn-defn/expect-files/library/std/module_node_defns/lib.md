Ok(
    [
        NodeDefn::Submodule(
            SubmoduleNodeDefn {
                node_decl: SubmoduleNodeDecl {
                    node_path: SubmoduleNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `std::prelude`,
                            disambiguator: 0,
                        },
                    },
                    ast_idx: 0,
                },
            },
        ),
        NodeDefn::Submodule(
            SubmoduleNodeDefn {
                node_decl: SubmoduleNodeDecl {
                    node_path: SubmoduleNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `std::logic`,
                            disambiguator: 0,
                        },
                    },
                    ast_idx: 1,
                },
            },
        ),
        NodeDefn::Submodule(
            SubmoduleNodeDefn {
                node_decl: SubmoduleNodeDecl {
                    node_path: SubmoduleNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `std::ops`,
                            disambiguator: 0,
                        },
                    },
                    ast_idx: 2,
                },
            },
        ),
    ],
)