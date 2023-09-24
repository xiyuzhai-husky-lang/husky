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
                    2,
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
                use_expr_idx: 5,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                        ident: `row_start`,
                        regional_token_idx: RegionalTokenIdx(
                            5,
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
                        ident: `row_end`,
                        regional_token_idx: RegionalTokenIdx(
                            9,
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
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FieldVariable {
                    ident_token: IdentRegionalToken {
                        ident: `upper_mass`,
                        regional_token_idx: RegionalTokenIdx(
                            13,
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
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::FieldVariable {
                    ident_token: IdentRegionalToken {
                        ident: `lower_mass`,
                        regional_token_idx: RegionalTokenIdx(
                            17,
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
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                        ident: `matches`,
                        regional_token_idx: RegionalTokenIdx(
                            5,
                        ),
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
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
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
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
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
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
                    ident: `ct`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                        ident: `mask`,
                        regional_token_idx: RegionalTokenIdx(
                            5,
                        ),
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryImage28`, `Struct`),
                    ),
                ),
            ),
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
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                        module_path: `mnist_classifier::connected_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
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
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `raw_contours`,
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
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `eff_holes`,
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
                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 2,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
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
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
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
            TokenInfo::Method,
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
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
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
            TokenInfo::Method,
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
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `max_hole_ilen`,
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    5,
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
            TokenInfo::Method,
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
                    5,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
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
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `max_row_span`,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    4,
                ),
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    4,
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
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `row_span_sum`,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    3,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `distribution`,
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
                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
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
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    28,
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    28,
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
                current_symbol_idx: 7,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 6,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
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
                    43,
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
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    43,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                    ),
                ),
            ),
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 5,
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
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `upper_mass`,
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `lower_mass`,
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `top_k_row_span_sum`,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `k`,
                },
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
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
                    14,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `k`,
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    14,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `top_k_row_right_mass_sum`,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `k`,
                },
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
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
                    14,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `k`,
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
            TokenInfo::SelfValue,
            TokenInfo::None,
            TokenInfo::Field,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    14,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
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
                    ident: `a`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `x`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `x`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `x`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `a`,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
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
                    ident: `a`,
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
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
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
                        TypePath(`mnist::BinaryImage28`, `Struct`),
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
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `img`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Method,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    7,
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
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    7,
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
                    7,
                ),
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 3,
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
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryImage28`, `Struct`),
                    ),
                ),
            ),
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    7,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                    ),
                ),
            ),
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
                current_symbol_idx: 5,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 4,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    7,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
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
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `img`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 9,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 8,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 10,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 9,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    7,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
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
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                    ident: `img`,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
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
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 11,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 10,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 8,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 7,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 12,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 11,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    7,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    104,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
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
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    104,
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 13,
                current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                    104,
                ),
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
            TokenInfo::Method,
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
                current_symbol_idx: 6,
                current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                    pattern_symbol_idx: 5,
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