Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Trait(
                                TraitSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Trait(
                                                TraitSynNodePathData {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TraitPath(`std::ops::Add`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Trait,
                            connection: Connected,
                        },
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `B`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
    },
)