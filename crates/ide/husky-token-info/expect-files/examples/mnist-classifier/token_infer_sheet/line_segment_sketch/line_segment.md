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
                                    submodule_item_path: SubmoduleItemPath(
                                        ItemPathId {
                                            data: ItemPathData::SubmoduleItem(
                                                SubmoduleItemPathData {
                                                    submodule_path: SubmodulePath(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                },
                                            ),
                                        },
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                    src: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentSynSymbolKind::FieldVariable {
                            ident_token: IdentRegionalToken {
                                ident: `start`,
                                regional_token_idx: RegionalTokenIdx(
                                    5,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::TemplateParameter(
                        2,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 2,
                        current_syn_symbol_kind: CurrentSynSymbolKind::FieldVariable {
                            ident_token: IdentRegionalToken {
                                ident: `end`,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                            AssociatedItemSynNodePath::TypeItem(
                                TypeItemSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::AssociatedItem(
                                            AssociatedItemSynNodePathData::TypeItem(
                                                TypeItemSynNodePathData {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodFn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        AssociatedItem {
                            associated_item_kind: TypeItem(
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
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                    data: TokenInfoData::SelfValue,
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
                            5,
                        ),
                    ),
                    data: TokenInfoData::Method,
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
                    data: TokenInfoData::SelfValue,
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
                    data: TokenInfoData::Field,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::AssociatedItem(
                            AssociatedItemSynNodePath::TypeItem(
                                TypeItemSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::AssociatedItem(
                                            AssociatedItemSynNodePathData::TypeItem(
                                                TypeItemSynNodePathData {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodFn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        AssociatedItem {
                            associated_item_kind: TypeItem(
                                MethodFn,
                            ),
                        },
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::PatternExpr(
                        1,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::PatternExpr(
                        1,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            1,
                        ),
                    ),
                    data: TokenInfoData::SelfValue,
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
                    data: TokenInfoData::Method,
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::PatternExpr(
                        2,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 2,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 2,
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
                            3,
                        ),
                    ),
                    data: TokenInfoData::SelfValue,
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
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 1,
                        inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                            ident: `pt`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            9,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 2,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            12,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 2,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 2,
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
                            13,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::PatternExpr(
                        3,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 3,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
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
                            14,
                        ),
                    ),
                    data: TokenInfoData::SelfValue,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            15,
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
                            17,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            16,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 1,
                        inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                            ident: `pt`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            18,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            20,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            19,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 3,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            23,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 3,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
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
                            24,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            25,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            27,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            26,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 2,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            28,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            29,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            30,
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