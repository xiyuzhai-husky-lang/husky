Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 2,
                rule_idx: OnceUseRuleIdx(
                    0,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::cmp::PartialEq`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                MajorItem {
                    module_item_kind: Trait,
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::cmp::Eq`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                MajorItem {
                    module_item_kind: Trait,
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::cmp::PartialOrd`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                MajorItem {
                    module_item_kind: Trait,
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::cmp::Ord`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                MajorItem {
                    module_item_kind: Trait,
                    connection: Connected,
                },
            ),
            TokenInfo::None,
        ],
    },
)