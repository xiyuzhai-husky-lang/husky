Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        3,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 3,
                        rule_idx: OnceUseRuleIdx(
                            0,
                        ),
                        state: OnceUseRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::CrateRoot {
                                    root_module_path: `mnist_classifier`,
                                },
                            ),
                        },
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        2,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 2,
                        rule_idx: OnceUseRuleIdx(
                            1,
                        ),
                        state: OnceUseRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::Submodule {
                                    submodule_path: SubmodulePath(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                },
                            ),
                        },
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Type(
                                Struct,
                            ),
                            connection: Connected,
                        },
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        1,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::FieldVariable {
                            ident_token: IdentRegionalToken {
                                ident: `line_segment_sketch`,
                                regional_token_idx: RegionalTokenIdx(
                                    4,
                                ),
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::SemaPrefixTypeOpr,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        2,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::FieldVariable {
                            ident_token: IdentRegionalToken {
                                ident: `line_segments`,
                                regional_token_idx: RegionalTokenIdx(
                                    9,
                                ),
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::SemaPrefixTypeOpr,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        ),
                    ),
                },
            ),
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
                                TraitPath(`core::visual::Visualize`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                            ),
                        ),
                    ),
                },
            ),
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
                                                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                ),
                                                disambiguator: 0,
                                            },
                                            ident: `visualize`,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::visual::Html`, `Extern`),
                            ),
                        ),
                    ),
                },
            ),
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            None,
        ],
    },
)