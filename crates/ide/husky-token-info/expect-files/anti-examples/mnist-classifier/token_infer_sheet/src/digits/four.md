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
                                    current_module_path: ModulePath(`mnist_classifier::digits::four`),
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
                                MajorFormSynNodePath(`mnist_classifier::digits::four::left_components`, `Val`, (0)),
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
                            ItemPath(`mnist_classifier::digits::four::left_components`),
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
                            ItemPath(`mnist_classifier::digits::four::left_components`),
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
                            ItemPath(`mnist_classifier::digits::four::left_components`),
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
                            ItemPath(`mnist_classifier::digits::four::left_components`),
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
                            ItemPath(`mnist_classifier::digits::four::left_components`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
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
                                MajorFormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::four::left_components`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
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
                                MajorFormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::four::left_components`),
                        ),
                        SemExprIdx(
                            5,
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
                                MajorFormSynNodePath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
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
                            ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
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
                            ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
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
                            ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
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
                            ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            1,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
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
                            3,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
                        ),
                        SemExprIdx(
                            1,
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
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::left_coordinate_max`),
                        ),
                        SemExprIdx(
                            2,
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
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`, (0)),
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
                            ItemPath(`mnist_classifier::digits::four::components_max_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::components_max_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::components_max_downwards`),
                        ),
                        SemExprIdx(
                            4,
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
                            ItemPath(`mnist_classifier::digits::four::components_max_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::components_max_downwards`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::displacement_downwards`, `Ritchie(
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
                                MajorFormPath(`mnist_classifier::digits::four::displacement_downwards`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::displacement_downwards`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::four::components_max_downwards`),
                        ),
                        SemExprIdx(
                            4,
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
                                MajorFormSynNodePath(`mnist_classifier::digits::four::components_max_heights`, `Val`, (0)),
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
                            ItemPath(`mnist_classifier::digits::four::components_max_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::components_max_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::components_max_heights`),
                        ),
                        SemExprIdx(
                            4,
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
                            ItemPath(`mnist_classifier::digits::four::components_max_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::components_max_heights`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::cc_box_heights`, `Ritchie(
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
                                MajorFormPath(`mnist_classifier::digits::four::cc_box_heights`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::cc_box_heights`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::four::components_max_heights`),
                        ),
                        SemExprIdx(
                            4,
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
                                MajorFormSynNodePath(`mnist_classifier::digits::four::is_four`, `Val`, (0)),
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
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            4,
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
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            5,
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
                            6,
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
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`mnist::MnistLabel::Four`),
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
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`mnist::MnistLabel::Four`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`mnist::MnistLabel::Four`),
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
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::left_components`, `Val`),
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
                                MajorFormPath(`mnist_classifier::digits::four::left_components`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::left_components`, `Val`),
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
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
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
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Literal,
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
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            3,
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
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
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
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::left_components`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::left_components`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::left_components`, `Val`),
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
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            17,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::Literal,
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
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            8,
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
                            21,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
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
                            26,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        4,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
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
                            28,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            10,
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
                            28,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
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
                            30,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            32,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            12,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
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
                            34,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            35,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            36,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            37,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            39,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        5,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            41,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        6,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
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
                            43,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            17,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            43,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        6,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
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
                            45,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            18,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            46,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            20,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            47,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            19,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            48,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            20,
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
                            50,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            21,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
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
                            52,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        7,
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
                            57,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        9,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
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
                            59,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            23,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 3,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            60,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            24,
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
                            62,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            25,
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
                            66,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            68,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        10,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
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
                            70,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            27,
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
                            70,
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
                            72,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            28,
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
                            74,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            29,
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
                            74,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        9,
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
                            76,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            30,
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
                            78,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            32,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
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
                            80,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            33,
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
                            82,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            35,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
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
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            36,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            85,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            38,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            86,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            37,
                        ),
                    ),
                    data: TokenInfoData::Literal,
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
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            38,
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
                            89,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        10,
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
                            92,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            40,
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
                            92,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        11,
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
                            94,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            41,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            42,
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
                            100,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        12,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 4,
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
                            102,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            44,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            102,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        12,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
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
                            104,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            45,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            105,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            47,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            106,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            46,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            107,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            47,
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
                            109,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            48,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 4,
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
                            111,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        13,
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
                            116,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            50,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            116,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        14,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
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
                            118,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            51,
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
                            120,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            122,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        15,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 5,
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
                            124,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            54,
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
                            124,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        15,
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
                            126,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            128,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            56,
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
                            128,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        16,
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
                            130,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            132,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        16,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
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
                            134,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            59,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            134,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        17,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
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
                            136,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            60,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            137,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            62,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            138,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            139,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            62,
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
                            141,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            63,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
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
                            143,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        18,
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
                            148,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            65,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            149,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            66,
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
                            151,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            67,
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
                            155,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            68,
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
                            157,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            159,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            71,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 6,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 10,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            160,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            72,
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
                            162,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            73,
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
                            165,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            74,
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
                            167,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            77,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            167,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        19,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
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
                            169,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
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
                            171,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            79,
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
                            173,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        19,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 7,
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
                            175,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            81,
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
                            175,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        20,
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
                            177,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            83,
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
                            179,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            82,
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
                            182,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            84,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 7,
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
                            184,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            85,
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
                            186,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            87,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 7,
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
                            188,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            88,
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
                            190,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        21,
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
                            192,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            90,
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
                            192,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        22,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            193,
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
                            195,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        SemExprIdx(
                            91,
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
                            195,
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
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::four::displacement_downwards`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                            ItemPath(`mnist_classifier::digits::four::displacement_downwards`),
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
                                MajorFormSynNodePath(`mnist_classifier::digits::four::cc_box_heights`, `Ritchie(
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
                        ),
                        SemExprIdx(
                            8,
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
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
                        ),
                        SemExprIdx(
                            9,
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
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
                        ),
                        SemExprIdx(
                            11,
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
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
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
                            29,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::four::cc_box_heights`),
                        ),
                        SemExprIdx(
                            13,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
        ],
    },
)
```