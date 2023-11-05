Ok(
    SynDeclSheet {
        [salsa id]: 55,
        decls: [
            (
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::MnistLabel`, `Enum`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::Enum(
                            EnumTypeSynDecl {
                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                template_parameters: [],
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryImage28`, `Struct`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::TupleStruct(
                            TupleStructTypeSynDecl {
                                path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                template_parameters: [],
                                fields: [
                                    TupleFieldSyndicate {
                                        decorators: [],
                                        visibility: None,
                                        ty: 4,
                                    },
                                ],
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            30,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 1,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 2,
                                                    argument_expr_idx: 3,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `r32`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: TupleStructFieldType,
                                                syn_expr_idx: 4,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::TupleStruct(
                            TupleStructTypeSynDecl {
                                path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                template_parameters: [],
                                fields: [
                                    TupleFieldSyndicate {
                                        decorators: [],
                                        visibility: None,
                                        ty: 4,
                                    },
                                ],
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::Literal(
                                                    RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    LiteralData::Integer(
                                                        UnspecifiedRegular(
                                                            31,
                                                        ),
                                                    ),
                                                ),
                                                SynExprData::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            syn_expr_idx: 1,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExprData::ExplicitApplication {
                                                    function_expr_idx: 2,
                                                    argument_expr_idx: 3,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `r32`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: TupleStructFieldType,
                                                syn_expr_idx: 4,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist::input`, `Val`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::Val(
                            ValFugitiveSynDecl {
                                path: FugitivePath(`mnist::input`, `Val`),
                                return_ty: Some(
                                    ReturnTypeBeforeEqSyndicate {
                                        expr: 1,
                                    },
                                ),
                                expr: None,
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist::input`, `Val`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist::BinaryImage28`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `BinaryImage28`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                syn_expr_idx: 1,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::ImplBlock(
                    ImplBlockPath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryImage28`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::TraitForType(
                        TraitForTypeImplBlockSynDecl {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            template_parameters: [],
                            trai_expr: TraitSyndicate {
                                expr: 1,
                            },
                            self_ty_decl: PathLeadingExpr(
                                SelfTypeSyndicate {
                                    expr: 2,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockSynNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `mnist`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::visual::Visualize`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Visualize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            2,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::visual::Visualize`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `BinaryImage28`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Trait,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SelfType,
                                            syn_expr_idx: 2,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TraitForTypeItem(
                        TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TraitForTypeItem(
                        TraitForTypeItemSynDecl::MethodFn(
                            TraitForTypeMethodFnSynDecl {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `mnist`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `visualize`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_value_parameter: None,
                                parenate_parameters: [],
                                return_ty: Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 1,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                TraitForTypeImplBlockSynNodePath {
                                                                    path: TraitForTypeImplBlockPath {
                                                                        module_path: `mnist`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Trait(
                                                                            TraitPath(`core::visual::Visualize`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `Visualize`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            2,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::visual::Visualize`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `BinaryImage28`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: SynPatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: Trait,
                                                            syn_expr_idx: 1,
                                                        },
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            syn_expr_idx: 2,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `mnist`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist::BinaryImage28`, `Struct`),
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
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::visual::Html`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `Html`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::visual::Html`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                syn_expr_idx: 1,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::ImplBlock(
                    ImplBlockPath::TypeImplBlock(
                        TypeImplBlockPath {
                            module_path: `mnist`,
                            ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::Type(
                        TypeImplBlockSynDecl {
                            path: TypeImplBlockPath {
                                module_path: `mnist`,
                                ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                disambiguator: 0,
                            },
                            template_parameters: [],
                            self_ty_expr: SelfTypeSyndicate {
                                expr: 1,
                            },
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TypeImplBlock(
                                                TypeImplBlockSynNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `mnist`,
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `BinaryImage28`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            2,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: SelfType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist`,
                                ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `new_zeros`,
                            item_kind: AssociatedFunctionFn,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::AssociatedFn(
                            TypeAssociatedFnSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist`,
                                        ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `new_zeros`,
                                    item_kind: AssociatedFunctionFn,
                                },
                                template_parameters: [],
                                parenate_parameters: [],
                                return_ty: Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 1,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TypeImplBlock(
                                                                TypeImplBlockSynNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist`,
                                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `BinaryImage28`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            2,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: SynPatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            syn_expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist`,
                                                                    ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `new_zeros`,
                                                                item_kind: AssociatedFunctionFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::SelfType(
                                                    RegionalTokenIdx(
                                                        8,
                                                    ),
                                                ),
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                syn_expr_idx: 1,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::ImplBlock(
                    ImplBlockPath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::ops::IntIndex`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryImage28`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::TraitForType(
                        TraitForTypeImplBlockSynDecl {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::ops::IntIndex`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            template_parameters: [],
                            trai_expr: TraitSyndicate {
                                expr: 1,
                            },
                            self_ty_decl: PathLeadingExpr(
                                SelfTypeSyndicate {
                                    expr: 2,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockSynNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `mnist`,
                                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::ops::IntIndex`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `core`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            2,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    `core`,
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 1,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        3,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ops`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        `core::ops`,
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 2,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `IntIndex`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::ops::IntIndex`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `BinaryImage28`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Trait,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SelfType,
                                            syn_expr_idx: 2,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TraitForTypeItem(
                        TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::ops::IntIndex`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `Output`,
                            item_kind: AssociatedType,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TraitForTypeItem(
                        TraitForTypeItemSynDecl::AssociatedType(
                            TraitForTypeAssociatedTypeSynDecl {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `mnist`,
                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Output`,
                                    item_kind: AssociatedType,
                                },
                                template_parameters: [],
                                ty_term_expr_idx: 1,
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                TraitForTypeImplBlockSynNodePath {
                                                                    path: TraitForTypeImplBlockPath {
                                                                        module_path: `mnist`,
                                                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 3,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Trait(
                                                                            TraitPath(`core::ops::IntIndex`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 4,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `core`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            2,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::Module(
                                                                    `core`,
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Subitem {
                                                                parent: 1,
                                                                colon_colon_token: ColonColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentRegionalToken {
                                                                        ident: `ops`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::Module(
                                                                        `core::ops`,
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Subitem {
                                                                parent: 2,
                                                                colon_colon_token: ColonColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentRegionalToken {
                                                                        ident: `IntIndex`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Trait(
                                                                            TraitPath(`core::ops::IntIndex`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `BinaryImage28`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: SynPatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: Trait,
                                                            syn_expr_idx: 1,
                                                        },
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            syn_expr_idx: 2,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `mnist`,
                                                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `Output`,
                                                                item_kind: AssociatedType,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `r32`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: AssociatedTypeTerm,
                                                syn_expr_idx: 1,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::ImplBlock(
                    ImplBlockPath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryGrid28`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::TraitForType(
                        TraitForTypeImplBlockSynDecl {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryGrid28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            template_parameters: [],
                            trai_expr: TraitSyndicate {
                                expr: 1,
                            },
                            self_ty_decl: PathLeadingExpr(
                                SelfTypeSyndicate {
                                    expr: 2,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockSynNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `mnist`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::visual::Visualize`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Visualize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            2,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::visual::Visualize`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `BinaryGrid28`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Trait,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SelfType,
                                            syn_expr_idx: 2,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TraitForTypeItem(
                        TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryGrid28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TraitForTypeItem(
                        TraitForTypeItemSynDecl::MethodFn(
                            TraitForTypeMethodFnSynDecl {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `mnist`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `visualize`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_value_parameter: None,
                                parenate_parameters: [],
                                return_ty: Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 1,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                TraitForTypeImplBlockSynNodePath {
                                                                    path: TraitForTypeImplBlockPath {
                                                                        module_path: `mnist`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Trait(
                                                                            TraitPath(`core::visual::Visualize`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 2,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `Visualize`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            2,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::visual::Visualize`),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `BinaryGrid28`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: SynPatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: Trait,
                                                            syn_expr_idx: 1,
                                                        },
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            syn_expr_idx: 2,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `mnist`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
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
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::visual::Html`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `Html`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::visual::Html`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                syn_expr_idx: 1,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::ImplBlock(
                    ImplBlockPath::TypeImplBlock(
                        TypeImplBlockPath {
                            module_path: `mnist`,
                            ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::Type(
                        TypeImplBlockSynDecl {
                            path: TypeImplBlockPath {
                                module_path: `mnist`,
                                ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                disambiguator: 0,
                            },
                            template_parameters: [],
                            self_ty_expr: SelfTypeSyndicate {
                                expr: 1,
                            },
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TypeImplBlock(
                                                TypeImplBlockSynNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `mnist`,
                                                        ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `BinaryGrid28`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            2,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: SelfType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist`,
                                ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `new_zeros`,
                            item_kind: AssociatedFunctionFn,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::AssociatedFn(
                            TypeAssociatedFnSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist`,
                                        ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `new_zeros`,
                                    item_kind: AssociatedFunctionFn,
                                },
                                template_parameters: [],
                                parenate_parameters: [],
                                return_ty: Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 1,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TypeImplBlock(
                                                                TypeImplBlockSynNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist`,
                                                                        ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `BinaryGrid28`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            2,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: SynPatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            syn_expr_idx: 1,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist`,
                                                                    ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `new_zeros`,
                                                                item_kind: AssociatedFunctionFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::SelfType(
                                                    RegionalTokenIdx(
                                                        8,
                                                    ),
                                                ),
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ReturnType,
                                                syn_expr_idx: 1,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::ImplBlock(
                    ImplBlockPath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::ops::IntIndex`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryGrid28`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::TraitForType(
                        TraitForTypeImplBlockSynDecl {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::ops::IntIndex`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryGrid28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            template_parameters: [],
                            trai_expr: TraitSyndicate {
                                expr: 1,
                            },
                            self_ty_decl: PathLeadingExpr(
                                SelfTypeSyndicate {
                                    expr: 2,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockSynNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `mnist`,
                                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::ops::IntIndex`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `core`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            2,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    `core`,
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 1,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        3,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ops`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        `core::ops`,
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 2,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `IntIndex`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::ops::IntIndex`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `BinaryGrid28`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Trait,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SelfType,
                                            syn_expr_idx: 2,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TraitForTypeItem(
                        TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::ops::IntIndex`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryGrid28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `Output`,
                            item_kind: AssociatedType,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TraitForTypeItem(
                        TraitForTypeItemSynDecl::AssociatedType(
                            TraitForTypeAssociatedTypeSynDecl {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `mnist`,
                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Output`,
                                    item_kind: AssociatedType,
                                },
                                template_parameters: [],
                                ty_term_expr_idx: 1,
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                TraitForTypeImplBlockSynNodePath {
                                                                    path: TraitForTypeImplBlockPath {
                                                                        module_path: `mnist`,
                                                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 3,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Trait(
                                                                            TraitPath(`core::ops::IntIndex`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExprData::PrincipalEntityPath {
                                                                path_expr_idx: 4,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_item_path_expr_arena: Arena {
                                                        data: [
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `core`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            2,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::Module(
                                                                    `core`,
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Subitem {
                                                                parent: 1,
                                                                colon_colon_token: ColonColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentRegionalToken {
                                                                        ident: `ops`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::Module(
                                                                        `core::ops`,
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Subitem {
                                                                parent: 2,
                                                                colon_colon_token: ColonColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                ),
                                                                ident_token: Ok(
                                                                    IdentRegionalToken {
                                                                        ident: `IntIndex`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                                path: Ok(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Trait(
                                                                            TraitPath(`core::ops::IntIndex`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynPrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `BinaryGrid28`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: SynPatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SynSymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: Trait,
                                                            syn_expr_idx: 1,
                                                        },
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            syn_expr_idx: 2,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `mnist`,
                                                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `Output`,
                                                                item_kind: AssociatedType,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                SynPrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `r32`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: AssociatedTypeTerm,
                                                syn_expr_idx: 1,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)