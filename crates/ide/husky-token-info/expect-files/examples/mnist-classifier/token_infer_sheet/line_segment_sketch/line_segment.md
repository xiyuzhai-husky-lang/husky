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
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: OnceUseRuleIdx(
                    1,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `mnist_classifier::geom2d`,
                            node: SubmoduleNode {
                                node_path: SubmoduleNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `mnist_classifier::geom2d`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 13,
                                ident_token: IdentToken {
                                    ident: `geom2d`,
                                    token_idx: TokenIdx(
                                        5,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Struct,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `dist_to_point`,
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
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `pt`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `pt`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)