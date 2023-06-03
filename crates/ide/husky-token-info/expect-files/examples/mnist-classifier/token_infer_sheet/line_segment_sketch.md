Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: UseExprRuleIdx(
                    0,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::concave_component`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 159,
                            ident_token: IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 3,
                rule_idx: UseExprRuleIdx(
                    1,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::convex_component`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 160,
                            ident_token: IdentToken {
                                ident: `convex_component`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 5,
                rule_idx: UseExprRuleIdx(
                    2,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::line_segment`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 162,
                            ident_token: IdentToken {
                                ident: `line_segment`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 8,
                rule_idx: UseExprRuleIdx(
                    3,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::CrateRoot {
                        root_module_path: `mnist_classifier`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 7,
                rule_idx: UseExprRuleIdx(
                    6,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::geom2d`,
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
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 11,
                rule_idx: UseExprRuleIdx(
                    4,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::CrateRoot {
                        root_module_path: `mnist_classifier`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 10,
                rule_idx: UseExprRuleIdx(
                    7,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::raw_contour`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 12,
                            ident_token: IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 9,
                rule_idx: UseExprRuleIdx(
                    8,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 203,
                            ident_token: IdentToken {
                                ident: `RawContour`,
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 13,
                rule_idx: UseExprRuleIdx(
                    5,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::line_segment`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 162,
                            ident_token: IdentToken {
                                ident: `line_segment`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::visual::Visualize`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                trai: TraitPath(`core::visual::Visualize`),
                                ident: `visualize`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::HtmlFunctionIdent,
            TokenInfo::HtmlPropertyIdent,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::HtmlPropertyIdent,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ident: `new`,
                                item_kind: AssociatedFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            AssociatedFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `from`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `to`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `from`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `to`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ident: `displacement`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::visual::Visualize`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                trai: TraitPath(`core::visual::Visualize`),
                                ident: `visualize`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::visual::Html`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ident: `concave_components`,
                                item_kind: MemoizedField,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ident: `bounding_box`,
                                item_kind: MemoizedField,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::FrameVariable(
                    15,
                ),
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::FrameVariable(
                    15,
                ),
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ident: `new`,
                                item_kind: AssociatedFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            AssociatedFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
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
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
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
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `u`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start`,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
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
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `end`,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `end`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `end`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `end`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start0`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `end`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start0`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start0`,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `start0`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
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
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
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
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ident: `new`,
                                item_kind: AssociatedFn,
                            },
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ident: `new`,
                                item_kind: AssociatedFn,
                            },
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `r`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ident: `new`,
                                item_kind: AssociatedFn,
                            },
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 13,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 13,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 13,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 14,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 14,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ident: `new`,
                                item_kind: AssociatedFn,
                            },
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 14,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 14,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
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
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 15,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 15,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 16,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 16,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 17,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 17,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 17,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 17,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 16,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 16,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 15,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 15,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 18,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 18,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ident: `new`,
                                item_kind: AssociatedFn,
                            },
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                    ident: `ct`,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 18,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 18,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 15,
                current_symbol_kind: CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: 15,
                },
                expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
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
        ],
    },
)