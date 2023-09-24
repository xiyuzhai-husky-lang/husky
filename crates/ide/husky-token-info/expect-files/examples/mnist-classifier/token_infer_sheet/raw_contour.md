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
                    4,
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
                    5,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::connected_component`,
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
                    6,
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
                use_expr_idx: 11,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::FieldVariable {
                    ident_token: IdentRegionalToken {
                        ident: `cc`,
                        regional_token_idx: RegionalTokenIdx(
                            5,
                        ),
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::FieldVariable {
                    ident_token: IdentRegionalToken {
                        ident: `points`,
                        regional_token_idx: RegionalTokenIdx(
                            10,
                        ),
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::visual::Visualize`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TraitForTypeItem(
                        TraitForTypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `mnist_classifier::raw_contour`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::visual::Html`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::raw_contour`,
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `line_segment_sketch`,
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
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::raw_contour`,
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    15,
                ),
                syn_expr_region: ExprRegionLeash(_),
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
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    15,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::raw_contour`,
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `relative_bounding_box`,
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
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
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
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::raw_contour`,
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `contour_len`,
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
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    3,
                ),
                syn_expr_region: ExprRegionLeash(_),
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
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    3,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    3,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
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
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
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
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::raw_contour`,
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
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
                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `start`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `end`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                MajorItem {
                    module_item_kind: Type(
                        Enum,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
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
                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
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
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
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
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `row`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `j`,
                },
                syn_expr_region: ExprRegionLeash(_),
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
                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
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
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
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
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `row`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `j`,
                },
                syn_expr_region: ExprRegionLeash(_),
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
                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
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
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
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
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `row`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `j`,
                },
                syn_expr_region: ExprRegionLeash(_),
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
                                path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
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
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 3,
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
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `row_above`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `j`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `row_below`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `j`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Left`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Up`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Down`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Right`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Left`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Up`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Right`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Up`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
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
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `outward`,
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
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `inward`,
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
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
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
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::raw_bits::r32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 3,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `row_above`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `j`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `row_below`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `j`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Down`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Left`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Right`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Down`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 4,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `inward_direction`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Down`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Left`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Up`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Right`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Left`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Up`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 4,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `inward_direction`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Left`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Up`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Right`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Down`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Right`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Down`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::CaseVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Unreachable,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::FieldVariable {
                    ident_token: IdentRegionalToken {
                        ident: `prev1`,
                        regional_token_idx: RegionalTokenIdx(
                            4,
                        ),
                    },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::FieldVariable {
                    ident_token: IdentRegionalToken {
                        ident: `prev2`,
                        regional_token_idx: RegionalTokenIdx(
                            8,
                        ),
                    },
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
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
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
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `points`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `points`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `points`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
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
                                path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
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
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    8,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `cc`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    8,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `cc`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    8,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    8,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    52,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    52,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    52,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    52,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `cc`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `cc`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 14,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 15,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 13,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 16,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 14,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 17,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 15,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 14,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 18,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 16,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 19,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 17,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 20,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 18,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 22,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 20,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 23,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 21,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 15,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 13,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 16,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 14,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 14,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 17,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 15,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 24,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 22,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 14,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 25,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 23,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 14,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 24,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 22,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 25,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 23,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 18,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 16,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 19,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 17,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 23,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 21,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 22,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 20,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 22,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 20,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 18,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 16,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 22,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 20,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 23,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 21,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 18,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 16,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 23,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 21,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 22,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 20,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 22,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 20,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 23,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 21,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 23,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 21,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 19,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 17,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 18,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 16,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 18,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 16,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 25,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 23,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 24,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 22,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Up`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `cc`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Down`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `cc`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Left`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::TypeVariant(
                    TypeVariantPath {
                        parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ident: `Right`,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 14,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 12,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 24,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 22,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 23,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 21,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 23,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 21,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 18,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 16,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 23,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 21,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 21,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 19,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `cc`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
        ],
    },
)