Ok(
    EntityTreePresheet {
        module_path: `std::ops`,
        node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Trait(
                                TraitNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`std::ops::Add`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `std::ops`,
                            ),
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Add`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Trait(
                            TraitNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitPath(`std::ops::Add`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Add`,
                    visibility: Scope::PubUnder(
                        `std::ops`,
                    ),
                },
            ],
        },
        use_one_trackers: UseExprRules(
            [],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [],
        },
        errors: [],
    },
)