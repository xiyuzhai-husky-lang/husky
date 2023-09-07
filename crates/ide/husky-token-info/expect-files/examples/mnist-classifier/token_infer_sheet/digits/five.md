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
                        EntitySymbol::SuperModule {
                            current_module_path: `mnist_classifier::digits::five`,
                            super_module_path: `mnist_classifier::digits`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                MajorItem {
                    module_item_kind: Fugitive(
                        Val,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::MnistLabel`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::MnistLabel`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                        ident: `Five`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                        ident: `Yes`,
                    },
                ),
            ),
        ],
    },
)