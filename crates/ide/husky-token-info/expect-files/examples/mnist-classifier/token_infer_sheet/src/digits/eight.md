```rust
Ok(
    TokenInfoSheet {
        token_infos_list: [
            [],
            [
                TokenInfo {
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
                                    current_module_path: ModulePath(`mnist_classifier::digits::eight`),
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
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`, (0)),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::eight::upper_mouth_match`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::upper_mouth_match`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::upper_mouth_match`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::upper_mouth_match`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::upper_mouth_match`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::upper_mouth_match`),
                        ),
                        SemExprIdx(
                            4,
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
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::eight::is_eight`, `Val`, (0)),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`mnist::MnistLabel::Eight`),
                        ),
                    ),
                },
                TokenInfo {
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`mnist::MnistLabel::Eight`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`mnist::MnistLabel::Eight`),
                        ),
                    ),
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            0,
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            2,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            5,
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            11,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            13,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            14,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [
                TokenInfo {
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
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            17,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        6,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::is_eight`),
                        ),
                        SemExprIdx(
                            19,
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        7,
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
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
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
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            5,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            9,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
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
            [],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            15,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            16,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            18,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            19,
                        ),
                    ),
                    data: TokenInfoData::Field,
                },
            ],
            [],
            [
                TokenInfo {
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`mnist_classifier::digits::eight::big_mouth`),
                        ),
                        SemExprIdx(
                            20,
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