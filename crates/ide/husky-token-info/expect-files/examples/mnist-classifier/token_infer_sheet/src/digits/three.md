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
                                    current_module_path: ModulePath(`mnist_classifier::digits::three`),
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
                                MajorFormSynNodePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`, (0)),
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
                            ItemPath(`mnist_classifier::digits::three::three_fermi_match`),
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
                            ItemPath(`mnist_classifier::digits::three::three_fermi_match`),
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
                            ItemPath(`mnist_classifier::digits::three::three_fermi_match`),
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
                            ItemPath(`mnist_classifier::digits::three::three_fermi_match`),
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
                            ItemPath(`mnist_classifier::digits::three::three_fermi_match`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
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
                                MajorFormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::three::three_fermi_match`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
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
                                MajorFormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::three::three_fermi_match`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::back`, `Ritchie(
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
                                MajorFormPath(`mnist_classifier::digits::three::back`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::back`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::three::three_fermi_match`),
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
                                MajorFormSynNodePath(`mnist_classifier::digits::three::is_three`, `Val`, (0)),
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
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
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
                            8,
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
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
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
                            9,
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
                            10,
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
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`mnist::MnistLabel::Three`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`mnist::MnistLabel::Three`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`mnist::MnistLabel::Three`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            0,
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
                            2,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
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
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            2,
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
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            4,
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
                            10,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            5,
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
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            6,
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
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
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
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
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
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
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
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
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
                            22,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            9,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            23,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            25,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            11,
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
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            27,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
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
                            29,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            12,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            29,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        5,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
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
                            31,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            13,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            32,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            15,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
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
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            14,
                        ),
                    ),
                    data: TokenInfoData::Literal,
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
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            15,
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
                            36,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        6,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::back`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::back`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            36,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
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
                            38,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            16,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            38,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        7,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
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
                            40,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            17,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            41,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            19,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            42,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            18,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            43,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            19,
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
                            45,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            20,
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
                            47,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        8,
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
                            52,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            22,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            53,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            23,
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
                            55,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            24,
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
                            57,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            25,
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
                            59,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            27,
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
                            61,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        9,
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
                            66,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        7,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 5,
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
                            68,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            29,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            69,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            30,
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
                            71,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            31,
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
                            75,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            33,
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
                            77,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            32,
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
                            80,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            34,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 5,
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
                            82,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
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
                            84,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            37,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 5,
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
                            87,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            38,
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
                            89,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        8,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 4,
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
                            91,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            42,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            92,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            43,
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
                            94,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            44,
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
                            98,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        9,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 5,
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
                            100,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            45,
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
                            101,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            46,
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
                            103,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            47,
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
                            107,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        10,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 8,
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
                            109,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            48,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 4,
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
                            111,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            50,
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
                            113,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            49,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 5,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 7,
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
                            116,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            51,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 8,
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
                            118,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            52,
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
                            120,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            54,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            120,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        10,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
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
                            122,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            55,
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
                            124,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            56,
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
                            126,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            58,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            127,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            59,
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
                            129,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            60,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            132,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            61,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            133,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        11,
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
                            135,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::is_three`),
                        ),
                        SemExprIdx(
                            64,
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
                            135,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        12,
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
                                MajorFormSynNodePath(`mnist_classifier::digits::three::uparc`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                            ItemPath(`mnist_classifier::digits::three::uparc`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::option::Option::Some`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::uparc`),
                        ),
                        SemExprIdx(
                            11,
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
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::uparc`),
                        ),
                        SemExprIdx(
                            7,
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
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::uparc`),
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
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::uparc`),
                        ),
                        SemExprIdx(
                            9,
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
                            25,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::uparc`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
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
                                MajorFormSynNodePath(`mnist_classifier::digits::three::downarc`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                            ItemPath(`mnist_classifier::digits::three::downarc`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::option::Option::Some`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::downarc`),
                        ),
                        SemExprIdx(
                            11,
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
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::downarc`),
                        ),
                        SemExprIdx(
                            7,
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
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::downarc`),
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
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::downarc`),
                        ),
                        SemExprIdx(
                            9,
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
                            25,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::downarc`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
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
                                MajorFormSynNodePath(`mnist_classifier::digits::three::back`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                            ItemPath(`mnist_classifier::digits::three::back`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::option::Option::Some`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::back`),
                        ),
                        SemExprIdx(
                            11,
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
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::back`),
                        ),
                        SemExprIdx(
                            7,
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
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::back`),
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
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::back`),
                        ),
                        SemExprIdx(
                            9,
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
                            25,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::three::back`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
        ],
    },
)
```