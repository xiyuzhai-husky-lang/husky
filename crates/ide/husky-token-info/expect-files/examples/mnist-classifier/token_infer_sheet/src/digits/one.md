```rust
Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        1,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 1,
                        rule_idx: OnceUseRuleIdx(
                            0,
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::RootSuperModule {
                                    current_module_path: ModulePath(`mnist_classifier::digits::one`),
                                    super_module_path: ModulePath(`mnist_classifier::digits`),
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
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Val,
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ),
                        ),
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
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                                    Fn,
                                )`),
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
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::hat`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::hat`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::one::is_one`, `Val`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Val,
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
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
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist::MnistLabel`, `Enum`),
                            ),
                        ),
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
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist::MnistLabel`, `Enum`),
                            ),
                        ),
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
                        3,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 278,
                                    },
                                ),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 278,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::Pattern(
                        0,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
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
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            12,
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
                        5,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            19,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        6,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            17,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            19,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        7,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            22,
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        8,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            26,
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
                        9,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
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
                        10,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 265,
                                    },
                                ),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 265,
                                    },
                                ),
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
                        11,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            31,
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
                        12,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
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
                    src: TokenInfoSource::Pattern(
                        1,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        14,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            38,
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
                    src: TokenInfoSource::Pattern(
                        2,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        16,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            42,
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
                    src: TokenInfoSource::Pattern(
                        3,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        18,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            46,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            49,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        19,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 218,
                                    },
                                ),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 218,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            51,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            52,
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
                        20,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            60,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            56,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            57,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            58,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            60,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        21,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            64,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        22,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            67,
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
                        23,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
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
                        24,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 265,
                                    },
                                ),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 265,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::Pattern(
                        5,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 4,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 4,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            72,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            74,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            76,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            78,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
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
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        25,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            82,
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
                        26,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            96,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        27,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            88,
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
                        28,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            90,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        29,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            92,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        30,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            94,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            96,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        31,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            113,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        32,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            100,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            104,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        33,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            106,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            110,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            111,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            113,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            115,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        34,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 217,
                                    },
                                ),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 217,
                                    },
                                ),
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            117,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            119,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            120,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            121,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            123,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            124,
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
                    src: TokenInfoSource::Pattern(
                        8,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 5,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 6,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            128,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            130,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            131,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            133,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            134,
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
                    src: TokenInfoSource::Pattern(
                        9,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            138,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 5,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 6,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            139,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            140,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            141,
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
                        35,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            156,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            145,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            147,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            148,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            150,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            151,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            153,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            154,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            156,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        36,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            167,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            159,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            160,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            161,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            162,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            163,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            164,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            167,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            169,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        37,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 217,
                                    },
                                ),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 217,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        38,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            192,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        39,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            173,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            177,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        40,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            179,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            183,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        41,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            185,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            189,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            190,
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
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            192,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            194,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 4,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 4,
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
                    src: TokenInfoSource::Pattern(
                        12,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 7,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 9,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            197,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            199,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            200,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            202,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            203,
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
                    src: TokenInfoSource::Pattern(
                        13,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 8,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            205,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 7,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 9,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            206,
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
                    src: TokenInfoSource::Pattern(
                        14,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 9,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 11,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            207,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            209,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            210,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            212,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            213,
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
                    src: TokenInfoSource::Pattern(
                        15,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 10,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 12,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            217,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 9,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 11,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            218,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        42,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            225,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            220,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 8,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            221,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            222,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 10,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 12,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            223,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            225,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            227,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 4,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 4,
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
                    src: TokenInfoSource::Pattern(
                        16,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 11,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 13,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        43,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            231,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::Pattern(
                        17,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 12,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 14,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        44,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            233,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::Pattern(
                        18,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 13,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 15,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            234,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 11,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 13,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            235,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 12,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 14,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::Pattern(
                        19,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 14,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 16,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            237,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 13,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 15,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            238,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 11,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 13,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            240,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 13,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 15,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            244,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 13,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 15,
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
                        45,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
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
                        46,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 265,
                                    },
                                ),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 265,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Ritchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::Pattern(
                        0,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::SemaPrefixTypeOpr,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        ),
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::SemaPrefixTypeOpr,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
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
                    src: TokenInfoSource::Pattern(
                        0,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 0,
                        inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                            ident: `cc`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            1,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            7,
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
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Ritchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::Pattern(
                        0,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::SemaPrefixTypeOpr,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        ),
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::SemaPrefixTypeOpr,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
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
                    src: TokenInfoSource::Pattern(
                        0,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 0,
                        inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                            ident: `cc`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            1,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            3,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 0,
                        inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                            ident: `cc`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            7,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
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
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::one::hat`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Ritchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::Pattern(
                        0,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::SemaPrefixTypeOpr,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        ),
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::SemaPrefixTypeOpr,
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
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
                    src: TokenInfoSource::Pattern(
                        0,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 0,
                        inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                            ident: `cc`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            1,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            3,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            7,
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
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            13,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemExpr(
                        SemExprIdx(
                            14,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ),
        ],
    },
)
```