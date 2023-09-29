Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::UseExpr {
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
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 2,
                rule_idx: OnceUseRuleIdx(
                    3,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 6,
                rule_idx: OnceUseRuleIdx(
                    1,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 5,
                rule_idx: OnceUseRuleIdx(
                    4,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 9,
                rule_idx: OnceUseRuleIdx(
                    2,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 8,
                rule_idx: OnceUseRuleIdx(
                    5,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Fn`),
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
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::basic::bool`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `line_segment_sketch`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `line_segment_sketch`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `index`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `line_segment_sketch`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `index`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `line_segment_sketch`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `index`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: SynCurrentSymbolKind::FrameVariable(
                    41,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `line_segment_sketch`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: SynCurrentSymbolKind::FrameVariable(
                    41,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `line_segment_sketch`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `index`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: SynCurrentSymbolKind::FrameVariable(
                    70,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {
                    ident: `line_segment_sketch`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: SynCurrentSymbolKind::FrameVariable(
                    70,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: SynCurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)