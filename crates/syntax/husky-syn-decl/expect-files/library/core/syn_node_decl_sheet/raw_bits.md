Ok(
    SynNodeDeclSheet {
        [salsa id]: 14,
        decls: [
            (
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::MajorItem(
                    MajorItemSynNodeDecl::Type(
                        TypeSynNodeDecl::Extern(
                            ExternTypeSynNodeDecl {
                                syn_node_path: TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::raw_bits::r32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 8,
                                template_parameter_decl_list: Ok(
                                    None,
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::raw_bits::r32`, `Extern`),
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
                                            pattern_infos: [],
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
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemSynNodePath::ImplBlock(
                    ImplBlockSynNodePath::TypeImplBlock(
                        TypeImplBlockSynNodePath {
                            path: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::ImplBlock(
                    ImplBlockSynNodeDecl::Type(
                        TypeImplBlockSynNodeDecl {
                            syn_node_path: TypeImplBlockSynNodePath {
                                path: TypeImplBlockPath {
                                    module_path: `core::raw_bits`,
                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 9,
                            impl_block: TypeImplBlockSynNode {
                                syn_node_path: TypeImplBlockSynNodePath {
                                    path: TypeImplBlockPath {
                                        module_path: `core::raw_bits`,
                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 9,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        28,
                                    ),
                                },
                                ty_expr: 45,
                                items: TypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        0..5,
                                    ),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    28,
                                ),
                            },
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            self_ty_expr: SelfTypeObelisk {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TypeImplBlock(
                                                TypeImplBlockSynNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `core::raw_bits`,
                                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 0,
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
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `r32`,
                                                        token_idx: TokenIdx(
                                                            29,
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
                                        pattern_infos: [],
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
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::raw_bits`,
                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `last_bits`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::AssociatedItem(
                    AssociatedItemSynNodeDecl::TypeItem(
                        TypeItemSynNodeDecl::MethodFn(
                            TypeMethodFnSynNodeDecl {
                                syn_node_path: TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `last_bits`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                node: TypeItemSynNode {
                                    syn_node_path: TypeItemSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `core::raw_bits`,
                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                                ident: `last_bits`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 0,
                                    ident: `last_bits`,
                                    item_kind: MethodFn,
                                    visibility: Scope::Pub,
                                    is_generic: false,
                                },
                                ast_idx: 0,
                                template_parameter_decl_list: Ok(
                                    None,
                                ),
                                ritchie_parameter_decl_list: Ok(
                                    RitchieParameters {
                                        lpar: LparToken(
                                            TokenIdx(
                                                34,
                                            ),
                                        ),
                                        self_value_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenate_parameters: [
                                            SpecificParameterObelisk::Regular {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        36,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RparToken(
                                            TokenIdx(
                                                38,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                39,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeBeforeColonObelisk {
                                            expr: 1,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Semicolon(
                                        EolSemicolonToken {
                                            token_idx: TokenIdx(
                                                41,
                                            ),
                                        },
                                    ),
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
                                                                        module_path: `core::raw_bits`,
                                                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
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
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `r32`,
                                                                        token_idx: TokenIdx(
                                                                            29,
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
                                                        pattern_infos: [],
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
                                                            expr_idx: 0,
                                                        },
                                                    ],
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
                                                                    module_path: `core::raw_bits`,
                                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `last_bits`,
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
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 1,
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
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                37,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `r32`,
                                                            token_idx: TokenIdx(
                                                                40,
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
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `k`,
                                                            token_idx: TokenIdx(
                                                                35,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    None,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    SynPatternSymbol::Atom(
                                                        0,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `k`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    None,
                                                ],
                                            },
                                        },
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            36,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `k`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 0,
                                                        ty_expr_idx: 0,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 0,
                                            },
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 1,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::raw_bits`,
                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `ctz`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::AssociatedItem(
                    AssociatedItemSynNodeDecl::TypeItem(
                        TypeItemSynNodeDecl::MethodFn(
                            TypeMethodFnSynNodeDecl {
                                syn_node_path: TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `ctz`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                node: TypeItemSynNode {
                                    syn_node_path: TypeItemSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `core::raw_bits`,
                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                                ident: `ctz`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 1,
                                    ident: `ctz`,
                                    item_kind: MethodFn,
                                    visibility: Scope::Pub,
                                    is_generic: false,
                                },
                                ast_idx: 1,
                                template_parameter_decl_list: Ok(
                                    None,
                                ),
                                ritchie_parameter_decl_list: Ok(
                                    RitchieParameters {
                                        lpar: LparToken(
                                            TokenIdx(
                                                45,
                                            ),
                                        ),
                                        self_value_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenate_parameters: [],
                                        commas: [],
                                        rpar: RparToken(
                                            TokenIdx(
                                                46,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                47,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeBeforeColonObelisk {
                                            expr: 0,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Semicolon(
                                        EolSemicolonToken {
                                            token_idx: TokenIdx(
                                                49,
                                            ),
                                        },
                                    ),
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
                                                                        module_path: `core::raw_bits`,
                                                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
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
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `r32`,
                                                                        token_idx: TokenIdx(
                                                                            29,
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
                                                        pattern_infos: [],
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
                                                            expr_idx: 0,
                                                        },
                                                    ],
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
                                                                    module_path: `core::raw_bits`,
                                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `ctz`,
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
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                48,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                            pattern_infos: [],
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
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::raw_bits`,
                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `co`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::AssociatedItem(
                    AssociatedItemSynNodeDecl::TypeItem(
                        TypeItemSynNodeDecl::MethodFn(
                            TypeMethodFnSynNodeDecl {
                                syn_node_path: TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `co`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                node: TypeItemSynNode {
                                    syn_node_path: TypeItemSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `core::raw_bits`,
                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                                ident: `co`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 2,
                                    ident: `co`,
                                    item_kind: MethodFn,
                                    visibility: Scope::Pub,
                                    is_generic: false,
                                },
                                ast_idx: 2,
                                template_parameter_decl_list: Ok(
                                    None,
                                ),
                                ritchie_parameter_decl_list: Ok(
                                    RitchieParameters {
                                        lpar: LparToken(
                                            TokenIdx(
                                                53,
                                            ),
                                        ),
                                        self_value_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenate_parameters: [],
                                        commas: [],
                                        rpar: RparToken(
                                            TokenIdx(
                                                54,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                55,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeBeforeColonObelisk {
                                            expr: 0,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Semicolon(
                                        EolSemicolonToken {
                                            token_idx: TokenIdx(
                                                57,
                                            ),
                                        },
                                    ),
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
                                                                        module_path: `core::raw_bits`,
                                                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
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
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `r32`,
                                                                        token_idx: TokenIdx(
                                                                            29,
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
                                                        pattern_infos: [],
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
                                                            expr_idx: 0,
                                                        },
                                                    ],
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
                                                                    module_path: `core::raw_bits`,
                                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `co`,
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
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                56,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                            pattern_infos: [],
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
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::raw_bits`,
                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `span`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::AssociatedItem(
                    AssociatedItemSynNodeDecl::TypeItem(
                        TypeItemSynNodeDecl::MethodFn(
                            TypeMethodFnSynNodeDecl {
                                syn_node_path: TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `span`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                node: TypeItemSynNode {
                                    syn_node_path: TypeItemSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `core::raw_bits`,
                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                                ident: `span`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 3,
                                    ident: `span`,
                                    item_kind: MethodFn,
                                    visibility: Scope::Pub,
                                    is_generic: false,
                                },
                                ast_idx: 3,
                                template_parameter_decl_list: Ok(
                                    None,
                                ),
                                ritchie_parameter_decl_list: Ok(
                                    RitchieParameters {
                                        lpar: LparToken(
                                            TokenIdx(
                                                61,
                                            ),
                                        ),
                                        self_value_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenate_parameters: [],
                                        commas: [],
                                        rpar: RparToken(
                                            TokenIdx(
                                                62,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                63,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeBeforeColonObelisk {
                                            expr: 0,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Semicolon(
                                        EolSemicolonToken {
                                            token_idx: TokenIdx(
                                                65,
                                            ),
                                        },
                                    ),
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
                                                                        module_path: `core::raw_bits`,
                                                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
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
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `r32`,
                                                                        token_idx: TokenIdx(
                                                                            29,
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
                                                        pattern_infos: [],
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
                                                            expr_idx: 0,
                                                        },
                                                    ],
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
                                                                    module_path: `core::raw_bits`,
                                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `span`,
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
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                64,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                            pattern_infos: [],
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
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::raw_bits`,
                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `right_mass`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                SynNodeDecl::AssociatedItem(
                    AssociatedItemSynNodeDecl::TypeItem(
                        TypeItemSynNodeDecl::MethodFn(
                            TypeMethodFnSynNodeDecl {
                                syn_node_path: TypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `right_mass`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                node: TypeItemSynNode {
                                    syn_node_path: TypeItemSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `core::raw_bits`,
                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                                ident: `right_mass`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 4,
                                    ident: `right_mass`,
                                    item_kind: MethodFn,
                                    visibility: Scope::Pub,
                                    is_generic: false,
                                },
                                ast_idx: 4,
                                template_parameter_decl_list: Ok(
                                    None,
                                ),
                                ritchie_parameter_decl_list: Ok(
                                    RitchieParameters {
                                        lpar: LparToken(
                                            TokenIdx(
                                                69,
                                            ),
                                        ),
                                        self_value_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenate_parameters: [],
                                        commas: [],
                                        rpar: RparToken(
                                            TokenIdx(
                                                70,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                71,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeBeforeColonObelisk {
                                            expr: 0,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Semicolon(
                                        EolSemicolonToken {
                                            token_idx: TokenIdx(
                                                73,
                                            ),
                                        },
                                    ),
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
                                                                        module_path: `core::raw_bits`,
                                                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 0,
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
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `r32`,
                                                                        token_idx: TokenIdx(
                                                                            29,
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
                                                        pattern_infos: [],
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
                                                            expr_idx: 0,
                                                        },
                                                    ],
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
                                                                    module_path: `core::raw_bits`,
                                                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `right_mass`,
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
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                72,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                            pattern_infos: [],
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
                                                expr_idx: 0,
                                            },
                                        ],
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