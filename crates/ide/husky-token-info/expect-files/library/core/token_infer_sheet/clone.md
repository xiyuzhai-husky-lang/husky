Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        2,
                    ),
                    data: TokenInfoData::UseExpr {
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
                },
            ),
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
                                TraitSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::clone::Clone`),
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                        ),
                    ),
                },
            ),
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
                        ItemSynNodePath::AssociatedItem(
                            AssociatedItemSynNodePath::TraitForTypeItem(
                                TraitForTypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `core::clone`,
                                                trai_path: TraitPath(`core::clone::Clone`),
                                                ty_sketch: TypeSketch::DeriveAny,
                                                disambiguator: 0,
                                            },
                                            ident: `clone`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        AssociatedItem {
                            associated_item_kind: TraitForTypeItem(
                                MethodFn,
                            ),
                        },
                    ),
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::SelfType,
                },
            ),
            None,
        ],
    },
)