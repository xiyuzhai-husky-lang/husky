```rust
Ok(
    TokenInfoSheet {
        token_infos_list: [
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::UseExpr(
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
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::one_fermi_match`),
                        ),
                        SemExprIdx(
                            0,
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
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            1,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::one_fermi_match`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            1,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            2,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::one_fermi_match`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            3,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::one_fermi_match`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            3,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::one_fermi_match`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::one_fermi_match`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::one_fermi_match`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::hat`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::hat`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::hat`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::one_fermi_match`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            0,
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
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            1,
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
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`mnist::MnistLabel::One`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`mnist::MnistLabel::One`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`mnist::MnistLabel::One`),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            1,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            1,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            2,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            3,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            3,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            13,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            19,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            26,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            28,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            12,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            30,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            13,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            32,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            15,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            32,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        5,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            33,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            19,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            34,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            16,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            34,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        6,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            36,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            17,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            40,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            18,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            42,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            19,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            45,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            21,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            45,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        7,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            47,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            22,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            49,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            23,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            52,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            25,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            52,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        8,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            54,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            26,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            56,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            27,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            57,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            59,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            29,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            59,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        10,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            63,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            30,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            63,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        11,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            65,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            31,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            67,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            32,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            69,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            34,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            69,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        12,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            71,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            35,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            73,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        13,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::downmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            73,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            75,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            37,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            75,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        14,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            77,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            38,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            78,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            40,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            79,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            39,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            80,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            40,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            82,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        15,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::upmost`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            82,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        2,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            84,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            41,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            84,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        16,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            86,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            42,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            87,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            44,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            88,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            43,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            89,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            44,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            91,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        17,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::hat`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::hat`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            91,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        3,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            93,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            45,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            93,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        18,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            95,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            46,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            96,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            48,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            97,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            47,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            98,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            48,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            100,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            49,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            102,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        19,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`core::option::Option::None`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::option::Option::None`),
                        ),
                    ),
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            105,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            51,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            107,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            52,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            109,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            53,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            110,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            55,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            110,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        20,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            111,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            60,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            112,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            56,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            114,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            57,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            116,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            58,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            122,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            59,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            124,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            60,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            127,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            62,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            129,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            63,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            129,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        21,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            131,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            64,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            133,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            66,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            133,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        22,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            135,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            67,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            137,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            69,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            139,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            141,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            71,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            141,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        24,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            143,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        5,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 4,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 4,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            145,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            72,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            146,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            73,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            148,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            74,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            150,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            75,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            154,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            76,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            155,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            77,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            157,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            78,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            159,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            79,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            163,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            81,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            163,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        25,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            165,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            82,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            167,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            83,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            168,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            85,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            168,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        26,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            169,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            96,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            170,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            86,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            170,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        27,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            172,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            88,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            174,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            87,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            177,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            89,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            177,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        28,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            179,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            90,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            181,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            91,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            181,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        29,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            183,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            92,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            185,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            93,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            185,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        30,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            187,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            94,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            191,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            95,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            193,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            96,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            195,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            98,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            195,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        31,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            196,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            113,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            197,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            99,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            197,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        32,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            199,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            100,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            200,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            102,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            201,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            101,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            202,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            102,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            203,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            103,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            205,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            104,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            207,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            105,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            207,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        33,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            209,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            106,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            210,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            108,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            211,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            107,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            212,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            108,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            213,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            109,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            215,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            110,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            217,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            111,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            223,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            112,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            225,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            113,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            228,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            115,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            230,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        34,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`core::option::Option::Some`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::option::Option::Some`),
                        ),
                    ),
                },
            ],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            236,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            117,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            237,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            118,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            239,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            119,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            241,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            120,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            245,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            121,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            246,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            122,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            248,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            123,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            250,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            124,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            254,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            126,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            256,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        8,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 5,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 6,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            258,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            128,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            259,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            129,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            261,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            130,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            262,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            137,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            263,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            131,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            264,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            132,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            266,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            133,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            268,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            134,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            272,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            135,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            273,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            137,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            275,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        9,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            277,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            138,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 5,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 6,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            279,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            139,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            283,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            140,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            285,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            141,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            287,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            142,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            288,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            144,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            288,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        35,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            289,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            156,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            290,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            145,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            291,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            146,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            293,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            147,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            295,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            148,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            296,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            149,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            298,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            150,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            300,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            151,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            301,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            152,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            303,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            153,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            305,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            154,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            311,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            155,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            313,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            156,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            315,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            158,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            315,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        36,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            316,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            167,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            317,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            159,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            319,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            160,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            323,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            161,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            325,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            162,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            327,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            163,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            329,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            164,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            333,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            166,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            335,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            167,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            338,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            169,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            340,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        37,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`core::option::Option::Some`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::option::Option::Some`),
                        ),
                    ),
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            345,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            171,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            345,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        38,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            346,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            192,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            347,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            172,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            347,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        39,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            349,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            173,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            350,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            175,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            351,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            174,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            352,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            175,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            353,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            176,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            355,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            177,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            357,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            178,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            357,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        40,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            359,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            179,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            360,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            181,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            361,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            180,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            362,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            181,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            363,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            182,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            365,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            183,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            367,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            184,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            367,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        41,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            369,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            185,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            370,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            187,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            371,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            186,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            372,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            187,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            373,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            188,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            375,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            189,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            377,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            190,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            383,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            191,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            385,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            192,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            388,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            194,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 4,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 4,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            390,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            195,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            392,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        12,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 7,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 9,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            394,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            197,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            395,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            198,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            397,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            199,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            398,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            204,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            399,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            200,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            400,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            201,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            402,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            202,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            404,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            203,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            407,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            204,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            409,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        13,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 8,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            411,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            205,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 7,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 9,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            413,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            206,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            417,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        14,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 9,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 11,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            419,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            207,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            420,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            208,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            422,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            209,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            423,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            216,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            424,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            210,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            425,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            211,
                        ),
                    ),
                    data: TokenInfoData::UnwrapExclamation,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            427,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            212,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            429,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            213,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            433,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            214,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            434,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            216,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            436,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        15,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 10,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 12,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            438,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            217,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 9,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 11,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            440,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            218,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            443,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            219,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            443,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        42,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            444,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            225,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            445,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            220,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 8,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            447,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            221,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            449,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            222,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 10,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 12,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            451,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            223,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            455,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            224,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            456,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            225,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            459,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            227,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 4,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 4,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            461,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            228,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            463,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        16,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 11,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 13,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            465,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            230,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            465,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        43,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            467,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            231,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            469,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        17,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 12,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 14,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            471,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            232,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            471,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        44,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            473,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            233,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            475,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        18,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 13,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 15,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            477,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            234,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 11,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 13,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            479,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            235,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 12,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 14,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            481,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        19,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 14,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 16,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            483,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            237,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 13,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 15,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            485,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            238,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 11,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 13,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            487,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            240,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 13,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 15,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            490,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            241,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            492,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            244,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 13,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 15,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            494,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            245,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            495,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            497,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::is_one`),
                        ),
                        SemExprIdx(
                            247,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            497,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        46,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::PrefixTypeOpr,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            0,
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
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::PrefixTypeOpr,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            2,
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
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            2,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 0,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `cc`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            17,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::upmost`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::PrefixTypeOpr,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            0,
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
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::PrefixTypeOpr,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            2,
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
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            2,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 0,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `cc`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 0,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `cc`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::downmost`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
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
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::PrefixTypeOpr,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            0,
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
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::PrefixTypeOpr,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            2,
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
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            2,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 0,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `cc`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            26,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            13,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            28,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::one::hat`),
                        ),
                        SemExprIdx(
                            14,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
        ],
    },
)
```