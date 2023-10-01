Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::Submodule(
                            SubmoduleSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: SubmodulePath(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        Module,
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::Submodule(
                            SubmoduleSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: SubmodulePath(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
                                    ),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        Module,
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::Submodule(
                            SubmoduleSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: SubmodulePath(
                                        `mnist_classifier::line_segment_sketch::convexity`,
                                    ),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        Module,
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::Submodule(
                            SubmoduleSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: SubmodulePath(
                                        `mnist_classifier::line_segment_sketch::line_segment`,
                                    ),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        Module,
                    ),
                },
            ),
            None,
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
                                EntitySymbol::Submodule {
                                    submodule_path: SubmodulePath(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
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
                    src: TokenInfoSource::UseExpr(
                        4,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 4,
                        rule_idx: OnceUseRuleIdx(
                            1,
                        ),
                        state: OnceUseRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::Submodule {
                                    submodule_path: SubmodulePath(
                                        `mnist_classifier::line_segment_sketch::convex_component`,
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
                    src: TokenInfoSource::UseExpr(
                        6,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 6,
                        rule_idx: OnceUseRuleIdx(
                            2,
                        ),
                        state: OnceUseRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::Submodule {
                                    submodule_path: SubmodulePath(
                                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                    src: TokenInfoSource::UseExpr(
                        9,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 9,
                        rule_idx: OnceUseRuleIdx(
                            3,
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
                        8,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 8,
                        rule_idx: OnceUseRuleIdx(
                            6,
                        ),
                        state: OnceUseRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::Submodule {
                                    submodule_path: SubmodulePath(
                                        `mnist_classifier::geom2d`,
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
                    src: TokenInfoSource::UseExpr(
                        12,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 12,
                        rule_idx: OnceUseRuleIdx(
                            4,
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
                        11,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 11,
                        rule_idx: OnceUseRuleIdx(
                            7,
                        ),
                        state: OnceUseRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::Submodule {
                                    submodule_path: SubmodulePath(
                                        `mnist_classifier::raw_contour`,
                                    ),
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
                        10,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 10,
                        rule_idx: OnceUseRuleIdx(
                            8,
                        ),
                        state: OnceUseRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
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
                        14,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 14,
                        rule_idx: OnceUseRuleIdx(
                            5,
                        ),
                        state: OnceUseRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::Submodule {
                                    submodule_path: SubmodulePath(
                                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                ident: `points`,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
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
                        2,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        2,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::FieldVariable {
                            ident_token: IdentRegionalToken {
                                ident: `start`,
                                regional_token_idx: RegionalTokenIdx(
                                    11,
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
                        3,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::FieldVariable {
                            ident_token: IdentRegionalToken {
                                ident: `points`,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            7,
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
                            9,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::FieldVariable {
                            ident_token: IdentRegionalToken {
                                ident: `end`,
                                regional_token_idx: RegionalTokenIdx(
                                    26,
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
                        4,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::FieldVariable {
                            ident_token: IdentRegionalToken {
                                ident: `points`,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            12,
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
                            14,
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
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::HtmlFunctionIdent,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::HtmlPropertyIdent,
                },
            ),
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
                    data: TokenInfoData::HtmlPropertyIdent,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            3,
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
                            4,
                        ),
                    ),
                    data: TokenInfoData::Field,
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
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::AssociatedItem(
                            AssociatedItemSynNodePath::TypeItem(
                                TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `new`,
                                            item_kind: AssociatedFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        AssociatedItem {
                            associated_item_kind: TypeItem(
                                AssociatedFn,
                            ),
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
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
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 2,
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
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
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
                        4,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `from`,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `to`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                            5,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            6,
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
                            11,
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
                            7,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `from`,
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
                            8,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `to`,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::AssociatedItem(
                            AssociatedItemSynNodePath::TypeItem(
                                TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `displacement`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
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
                    data: TokenInfoData::SelfType,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                ident: `contour`,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                ident: `strokes`,
                                regional_token_idx: RegionalTokenIdx(
                                    10,
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
                            3,
                        ),
                    ),
                    data: TokenInfoData::VecFunctorBoxPrefix,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::VecFunctorBoxPrefix,
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
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
            None,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::AssociatedItem(
                            AssociatedItemSynNodePath::TypeItem(
                                TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `concave_components`,
                                            item_kind: MemoizedField,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        AssociatedItem {
                            associated_item_kind: TypeItem(
                                MemoizedField,
                            ),
                        },
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
                    data: TokenInfoData::VecFunctorBoxPrefix,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::VecFunctorBoxPrefix,
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
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
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
                            2,
                        ),
                    ),
                    data: TokenInfoData::SelfType,
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
                                TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `bounding_box`,
                                            item_kind: MemoizedField,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        AssociatedItem {
                            associated_item_kind: TypeItem(
                                MemoizedField,
                            ),
                        },
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        1,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        2,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            6,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            7,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            8,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                    data: TokenInfoData::Field,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        4,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            10,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            11,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        5,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                            12,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            13,
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
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::FrameVariable(
                            16,
                        ),
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
                    data: TokenInfoData::SelfType,
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
                            16,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        7,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            18,
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
                            19,
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
                            20,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::FrameVariable(
                            16,
                        ),
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
                            22,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            23,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            24,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            25,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            26,
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
                            29,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            30,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            33,
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
                            31,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            32,
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
                            35,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            36,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            39,
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
                            37,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            38,
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
                            41,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                            42,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                            45,
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
                            43,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            44,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
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
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                            49,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            50,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                            53,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            54,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::AssociatedItem(
                            AssociatedItemSynNodePath::TypeItem(
                                TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `new`,
                                            item_kind: AssociatedFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        AssociatedItem {
                            associated_item_kind: TypeItem(
                                AssociatedFn,
                            ),
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
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
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 2,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
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
                            4,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            5,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
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
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Fugitive(
                                Fn,
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        2,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 2,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        1,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            1,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            7,
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
                            8,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                    data: TokenInfoData::Field,
                },
            ),
            None,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            14,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            15,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            17,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                            18,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            20,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            21,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            23,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            30,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            31,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            32,
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
                            34,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        4,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            36,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            38,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            39,
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
                            41,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                            44,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            45,
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
                            46,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            48,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            49,
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
                            50,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Fugitive(
                                Fn,
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        2,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 2,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        1,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            1,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            7,
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
                            8,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                    data: TokenInfoData::Field,
                },
            ),
            None,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            14,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            15,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            17,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                            18,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            20,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            21,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            23,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
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
                            30,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            32,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            33,
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
                            35,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        4,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            37,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            38,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            39,
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
                            41,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                            44,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            45,
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
                            46,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            48,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `u`,
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
                            49,
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
                            50,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Fugitive(
                                Fn,
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
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
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 2,
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
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        1,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        2,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            2,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            7,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start`,
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
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            8,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            10,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        4,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            11,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start`,
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
                            12,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            15,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            17,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            18,
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
                            19,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                            22,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            24,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            25,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            26,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start`,
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
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            32,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            33,
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
                            34,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                            36,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        5,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
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
                            38,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            39,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        6,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
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
                            42,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            43,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        7,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            46,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            47,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            49,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                            51,
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
                            50,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            55,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            57,
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
                            56,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        8,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            61,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            62,
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
                            63,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            64,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            65,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            68,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            69,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            71,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            72,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            74,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            75,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        9,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 9,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 9,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
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
                            78,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            79,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        10,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
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
                            82,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            83,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                            85,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                            87,
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
                            86,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 9,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 9,
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
                            90,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                            91,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 9,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 9,
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
                            93,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
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
                            95,
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
                            94,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            98,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            99,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            101,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            103,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            104,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            109,
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
                            105,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start`,
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
                            106,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            111,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            112,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start`,
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
                            114,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Fugitive(
                                Fn,
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
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
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 2,
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
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        4,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 4,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        5,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        1,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `end`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        2,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            2,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            7,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `end`,
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
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            8,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `end`,
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
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            10,
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
                            11,
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
                            13,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            14,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            16,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            17,
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
                            18,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 4,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                            21,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            23,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            24,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            29,
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
                            25,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `end`,
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
                            26,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                            31,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            32,
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
                            33,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 4,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                            35,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            37,
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
                            36,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start0`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        4,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
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
                            39,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            40,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 4,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        5,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
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
                            43,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            44,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 4,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        6,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            47,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            48,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        7,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            50,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            55,
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
                            51,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 3,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `end`,
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
                            52,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        8,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            56,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            57,
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
                            58,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            59,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            60,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 4,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            63,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            64,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            66,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            67,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            69,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            70,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 4,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        9,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 9,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 9,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
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
                            73,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            74,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 4,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        10,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
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
                            77,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            78,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 4,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
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
                            80,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            82,
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
                            81,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 9,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 9,
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
                            85,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            86,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 9,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 9,
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
                            88,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
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
                            90,
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
                            89,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                            93,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                            94,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
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
                            96,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            98,
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
                            97,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            101,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            102,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start0`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            104,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            106,
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
                            105,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            109,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            111,
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
                            110,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            118,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            120,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            121,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start0`,
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
                            123,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            124,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `start0`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Fugitive(
                                Fn,
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
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
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                        current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                            pattern_symbol_idx: 2,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::VecFunctorBoxPrefix,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::VecFunctorBoxPrefix,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        1,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                    data: TokenInfoData::VecFunctorBoxPrefix,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::VecFunctorBoxPrefix,
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
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        2,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        3,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        4,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            7,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            8,
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
                            9,
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
                            10,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            11,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
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
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
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
                            15,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            16,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            17,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        5,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            21,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            22,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            23,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        6,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            26,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
            None,
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        7,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 5,
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
                            31,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        8,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            32,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            33,
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
                            35,
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
                            36,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            38,
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
                            37,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
                            39,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            42,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 7,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 7,
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
                            44,
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
                            43,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 8,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 8,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        9,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 9,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 9,
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
                            48,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            49,
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
                            50,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            51,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            52,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            55,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            56,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            57,
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
                            59,
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
                            60,
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
                            61,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            64,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            67,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 6,
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
                            68,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        5,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
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
                            70,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            71,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            72,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            73,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 2,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `r`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        10,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        6,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            77,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            78,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            79,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
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
                            81,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            82,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        11,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 11,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 11,
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
                            85,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            86,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        12,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 12,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 12,
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
                            88,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 11,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 11,
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
                            89,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        13,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 13,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 13,
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
                            90,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
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
                            91,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        14,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 14,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 14,
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
                            92,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 11,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 11,
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
                            93,
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
                            96,
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
                            94,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
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
                            95,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            97,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 13,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 13,
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
                            99,
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
                            98,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 12,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 12,
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
                            100,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            103,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 13,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 13,
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
                            105,
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
                            104,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 12,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 12,
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
                            109,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 13,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 13,
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
                            111,
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
                            110,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 14,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 14,
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
                            112,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            116,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 13,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 13,
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
                            118,
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
                            117,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 14,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 14,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        15,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 15,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 15,
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
                            122,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            123,
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
                            125,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        7,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            127,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            128,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 15,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 15,
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
                            129,
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
                            130,
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
                            131,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
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
                            132,
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
                            133,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            136,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 4,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 4,
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
                            137,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            138,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            139,
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
                            140,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            143,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            145,
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
                            144,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
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
                            146,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            147,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            149,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 3,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            150,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 2,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        16,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 16,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 16,
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
                            154,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            155,
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
                            156,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        17,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 17,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 17,
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
                            157,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            158,
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
                            160,
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
                            161,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        18,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 18,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 18,
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
                            162,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            163,
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
                            165,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 18,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 18,
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
                            166,
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
                            167,
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
                            168,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 17,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 17,
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
                            169,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 16,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 16,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        19,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 19,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 19,
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
                            172,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            173,
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
                            175,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            176,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        8,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            179,
                        ),
                    ),
                    data: TokenInfoData::InheritedSymbol {
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `ct`,
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
                            180,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 19,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 19,
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
                            181,
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
                            182,
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
                            183,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 16,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 16,
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
                            185,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
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
                            186,
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
                            188,
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
                            189,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            194,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 1,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
        ],
    },
)