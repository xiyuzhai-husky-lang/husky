Ok(
    [
        SynNodeDefn::Submodule(
            SubmoduleSynNodeDefn {
                syn_node_decl: SubmoduleNodeDecl {
                    syn_node_path: SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `std::prelude`,
                            disambiguator: 0,
                        },
                    },
                    ast_idx: 0,
                },
            },
        ),
        SynNodeDefn::Submodule(
            SubmoduleSynNodeDefn {
                syn_node_decl: SubmoduleNodeDecl {
                    syn_node_path: SubmoduleSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: `std::logic`,
                            disambiguator: 0,
                        },
                    },
                    ast_idx: 1,
                },
            },
        ),
        SynNodeDefn::Submodule(
            SubmoduleSynNodeDefn {
                syn_node_decl: SubmoduleNodeDecl {
                    syn_node_path: SubmoduleSynNodePath {
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