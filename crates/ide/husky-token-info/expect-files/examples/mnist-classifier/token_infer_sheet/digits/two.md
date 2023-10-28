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
                                EntitySymbol::SuperModule {
                                    current_module_path: `mnist_classifier::digits::two`,
                                    super_module_path: `mnist_classifier::digits`,
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
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
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
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
                            ),
                        ),
                    ),
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
                                FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
                            ),
                        ),
                    ),
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
                                FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
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
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Fugitive(
                                FunctionFn,
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
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `cc`,
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
                            3,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            7,
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
                            8,
                        ),
                    ),
                    data: TokenInfoData::Field,
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
                                        path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Fugitive(
                                FunctionFn,
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
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `cc`,
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
                            3,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            7,
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
                            8,
                        ),
                    ),
                    data: TokenInfoData::Field,
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
                                        path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        MajorItem {
                            module_item_kind: Fugitive(
                                FunctionFn,
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
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        inherited_symbol_idx: 1,
                        inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                            ident: `cc`,
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
                            3,
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
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            7,
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
                            8,
                        ),
                    ),
                    data: TokenInfoData::Field,
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
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                                TypePath(`mnist::MnistLabel`, `Enum`),
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
                                TypePath(`mnist::MnistLabel`, `Enum`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            TypeVariantPath {
                                parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                ident: `Two`,
                            },
                        ),
                    ),
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
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            ),
                        ),
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
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
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
                        2,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
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
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
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
                        3,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
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
                        4,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
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
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
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
                        5,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        5,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 5,
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
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
                        6,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        6,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 6,
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
                            pattern_symbol_idx: 6,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        7,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                            14,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        8,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                            16,
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
                            18,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        9,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 9,
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
                            pattern_symbol_idx: 9,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
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
                        9,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                            23,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        10,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                            27,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        11,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                            31,
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
                            34,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        12,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        13,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                            40,
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
                            42,
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
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            45,
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
                            48,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        14,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 14,
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
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
                            50,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        15,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 15,
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
                            pattern_symbol_idx: 15,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
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
                            52,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            54,
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
                            55,
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
                            59,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            61,
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
                            64,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            66,
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
                            67,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
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
                            68,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            70,
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
                            71,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        20,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 20,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 20,
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
                        current_symbol_idx: 10,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 10,
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
                            74,
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
                            75,
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
                        21,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 21,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 21,
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
                            76,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            78,
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
                            79,
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
                        22,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 22,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 22,
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
                        current_symbol_idx: 20,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 20,
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
                            81,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 21,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 21,
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
                        23,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 23,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 23,
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
                        current_symbol_idx: 11,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 11,
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
                            88,
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
                        24,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 24,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 24,
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
                        current_symbol_idx: 11,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 11,
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
                            92,
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
                            93,
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
                        25,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 25,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 25,
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
                            94,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 23,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 23,
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
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 24,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 24,
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
                            100,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 22,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 22,
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
                            101,
                        ),
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 25,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 25,
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
                            106,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        26,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 26,
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
                            pattern_symbol_idx: 26,
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
                            108,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        27,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 27,
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
                            pattern_symbol_idx: 27,
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
                            110,
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
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynCurrentSymbol(
                        28,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 28,
                        current_symbol_kind: SynCurrentSymbolKind::BeVariable {
                            pattern_symbol_idx: 28,
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
                            114,
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
                            115,
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
                    src: TokenInfoSource::SynCurrentSymbol(
                        29,
                    ),
                    data: TokenInfoData::CurrentSymbol {
                        current_symbol_idx: 29,
                        current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                            pattern_symbol_idx: 29,
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
                            120,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        14,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        15,
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            TypeVariantPath {
                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                ident: `Yes`,
                            },
                        ),
                    ),
                },
            ),
        ],
    },
)