Ok(
    [
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Type(
                TypeSynNodeDefn::Extern(
                    ExternTypeSynNodeDefn {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::slice::Slice`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: ExternTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::slice::Slice`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 4,
                            template_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LaOrLtToken(
                                            TokenIdx(
                                                7,
                                            ),
                                        ),
                                        template_parameters: [
                                            TemplateParameterDecl {
                                                annotated_variance_token: Some(
                                                    VarianceToken::Covariant(
                                                        CovariantToken {
                                                            token_idx: TokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                symbol: 0,
                                                variant: TemplateParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtToken(
                                            TokenIdx(
                                                10,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`core::slice::Slice`, `Extern`),
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
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: Some(
                                                            VarianceToken::Covariant(
                                                                CovariantToken {
                                                                    token_idx: TokenIdx(
                                                                        8,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    9,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Type(
                TypeSynNodeDefn::Extern(
                    ExternTypeSynNodeDefn {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: ExternTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 5,
                            template_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LaOrLtToken(
                                            TokenIdx(
                                                15,
                                            ),
                                        ),
                                        template_parameters: [
                                            TemplateParameterDecl {
                                                annotated_variance_token: Some(
                                                    VarianceToken::Covariant(
                                                        CovariantToken {
                                                            token_idx: TokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                symbol: 0,
                                                variant: TemplateParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            17,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtToken(
                                            TokenIdx(
                                                18,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        18,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: Some(
                                                            VarianceToken::Covariant(
                                                                CovariantToken {
                                                                    token_idx: TokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    17,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::ImplBlock(
            ImplBlockSynNodeDecl::Type(
                TypeImplBlockSynNodeDecl {
                    syn_node_path: TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                    ast_idx: 6,
                    impl_block: TypeImplBlockSynNode {
                        syn_node_path: TypeImplBlockSynNodePath {
                            path: TypeImplBlockPath {
                                module_path: `core::slice`,
                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 6,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                20,
                            ),
                        },
                        ty_expr: 51,
                        items: TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                0..3,
                            ),
                        },
                    },
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            20,
                        ),
                    },
                    template_parameter_decl_list: Ok(
                        Some(
                            Generics {
                                langle: LaOrLtToken(
                                    TokenIdx(
                                        21,
                                    ),
                                ),
                                template_parameters: [
                                    TemplateParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `E`,
                                                token_idx: TokenIdx(
                                                    22,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                ],
                                commas: [],
                                decl_list_result: Ok(
                                    (),
                                ),
                                rangle: RaOrGtToken(
                                    TokenIdx(
                                        23,
                                    ),
                                ),
                            },
                        ),
                    ),
                    self_ty_expr: SelfTypeExpr {
                        expr: 2,
                    },
                    eol_colon: Ok(
                        EolToken::Colon(
                            EolColonToken {
                                token_idx: TokenIdx(
                                    26,
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
                                                module_path: `core::slice`,
                                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                    TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `E`,
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                        current_symbol_idx: 0,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `E`,
                                                    token_idx: TokenIdx(
                                                        22,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 0,
                                        argument_expr_idx: 1,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `CyclicSliceLeashed`,
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                    data: [
                                        CurrentSynSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                23,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                },
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [
                                    (
                                        TemplateTypeParameter,
                                        ArenaIdxRange(
                                            0..1,
                                        ),
                                    ),
                                ],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: SelfType,
                                    expr_idx: 2,
                                },
                            ],
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MethodFn(
                    TypeMethodFnSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `ilen`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `core::slice`,
                                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                            disambiguator: 0,
                                        },
                                        ident: `ilen`,
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
                                                module_path: `core::slice`,
                                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `ilen`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 0,
                                ident: `ilen`,
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
                                            30,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparToken(
                                        TokenIdx(
                                            31,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            32,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 0,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Semicolon(
                                    EolSemicolonToken {
                                        token_idx: TokenIdx(
                                            34,
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
                                                                    module_path: `core::slice`,
                                                                    ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                                        TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                25,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E`,
                                                                        token_idx: TokenIdx(
                                                                            22,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 0,
                                                            argument_expr_idx: 1,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `CyclicSliceLeashed`,
                                                                    token_idx: TokenIdx(
                                                                        24,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    23,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E`,
                                                                            token_idx: TokenIdx(
                                                                                22,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 2,
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
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `ilen`,
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
                                                            33,
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
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
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
                        body: None,
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
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
                                                                            module_path: `core::slice`,
                                                                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                                                TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        25,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    22,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 0,
                                                                    argument_expr_idx: 1,
                                                                },
                                                            ],
                                                        },
                                                        principal_item_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `CyclicSliceLeashed`,
                                                                            token_idx: TokenIdx(
                                                                                24,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                                data: [
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            23,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E`,
                                                                                    token_idx: TokenIdx(
                                                                                        22,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: False,
                                                            pattern_ty_constraints: [
                                                                (
                                                                    TemplateTypeParameter,
                                                                    ArenaIdxRange(
                                                                        0..1,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            SynExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 2,
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
                                                                        module_path: `core::slice`,
                                                                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `ilen`,
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
                                                                    33,
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
                                                    data: [
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `E`,
                                                                },
                                                            ),
                                                        },
                                                    ],
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
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::AssociatedItem(
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `ilen`,
                                                        item_kind: MethodFn,
                                                    },
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: True,
                                    pattern_ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MethodFn(
                    TypeMethodFnSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `first`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `core::slice`,
                                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                            disambiguator: 0,
                                        },
                                        ident: `first`,
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
                                                module_path: `core::slice`,
                                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `first`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 1,
                                ident: `first`,
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
                                            38,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparToken(
                                        TokenIdx(
                                            39,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            40,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 2,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Semicolon(
                                    EolSemicolonToken {
                                        token_idx: TokenIdx(
                                            44,
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
                                                                    module_path: `core::slice`,
                                                                    ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                                        TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                25,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E`,
                                                                        token_idx: TokenIdx(
                                                                            22,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 0,
                                                            argument_expr_idx: 1,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `CyclicSliceLeashed`,
                                                                    token_idx: TokenIdx(
                                                                        24,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    23,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E`,
                                                                            token_idx: TokenIdx(
                                                                                22,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 2,
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
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `first`,
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
                                            SynExpr::InheritedSymbol {
                                                ident: `E`,
                                                token_idx: TokenIdx(
                                                    43,
                                                ),
                                                inherited_symbol_idx: 0,
                                                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    42,
                                                ),
                                                opd: 0,
                                            },
                                            SynExpr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    41,
                                                ),
                                                opd: 1,
                                            },
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
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
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
                                            expr_idx: 2,
                                        },
                                    ],
                                },
                            },
                        },
                        body: None,
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
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
                                                                            module_path: `core::slice`,
                                                                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                                                TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        25,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    22,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 0,
                                                                    argument_expr_idx: 1,
                                                                },
                                                            ],
                                                        },
                                                        principal_item_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `CyclicSliceLeashed`,
                                                                            token_idx: TokenIdx(
                                                                                24,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                                data: [
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            23,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E`,
                                                                                    token_idx: TokenIdx(
                                                                                        22,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: False,
                                                            pattern_ty_constraints: [
                                                                (
                                                                    TemplateTypeParameter,
                                                                    ArenaIdxRange(
                                                                        0..1,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            SynExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 2,
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
                                                                        module_path: `core::slice`,
                                                                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `first`,
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
                                                    SynExpr::InheritedSymbol {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            43,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E`,
                                                            },
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            42,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            41,
                                                        ),
                                                        opd: 1,
                                                    },
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
                                                    data: [
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `E`,
                                                                },
                                                            ),
                                                        },
                                                    ],
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
                                                    expr_idx: 2,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::AssociatedItem(
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `first`,
                                                        item_kind: MethodFn,
                                                    },
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: True,
                                    pattern_ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TypeItem(
                TypeItemSynNodeDefn::MethodFn(
                    TypeMethodFnSynNodeDefn {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `last`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `core::slice`,
                                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                            disambiguator: 0,
                                        },
                                        ident: `last`,
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
                                                module_path: `core::slice`,
                                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `last`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 2,
                                ident: `last`,
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
                                            48,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparToken(
                                        TokenIdx(
                                            49,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            50,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 2,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Semicolon(
                                    EolSemicolonToken {
                                        token_idx: TokenIdx(
                                            54,
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
                                                                    module_path: `core::slice`,
                                                                    ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                                        TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                25,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E`,
                                                                        token_idx: TokenIdx(
                                                                            22,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 0,
                                                            argument_expr_idx: 1,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `CyclicSliceLeashed`,
                                                                    token_idx: TokenIdx(
                                                                        24,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    23,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E`,
                                                                            token_idx: TokenIdx(
                                                                                22,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 2,
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
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `last`,
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
                                            SynExpr::InheritedSymbol {
                                                ident: `E`,
                                                token_idx: TokenIdx(
                                                    53,
                                                ),
                                                inherited_symbol_idx: 0,
                                                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    52,
                                                ),
                                                opd: 0,
                                            },
                                            SynExpr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    51,
                                                ),
                                                opd: 1,
                                            },
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
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
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
                                            expr_idx: 2,
                                        },
                                    ],
                                },
                            },
                        },
                        body: None,
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
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
                                                                            module_path: `core::slice`,
                                                                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                                                TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        25,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    22,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 0,
                                                                    argument_expr_idx: 1,
                                                                },
                                                            ],
                                                        },
                                                        principal_item_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `CyclicSliceLeashed`,
                                                                            token_idx: TokenIdx(
                                                                                24,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                                                data: [
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            23,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E`,
                                                                                    token_idx: TokenIdx(
                                                                                        22,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: False,
                                                            pattern_ty_constraints: [
                                                                (
                                                                    TemplateTypeParameter,
                                                                    ArenaIdxRange(
                                                                        0..1,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            SynExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 2,
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
                                                                        module_path: `core::slice`,
                                                                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `last`,
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
                                                    SynExpr::InheritedSymbol {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E`,
                                                            },
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                        opd: 1,
                                                    },
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
                                                    data: [
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `E`,
                                                                },
                                                            ),
                                                        },
                                                    ],
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
                                                    expr_idx: 2,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::AssociatedItem(
                                        AssociatedItemSynNodePath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `last`,
                                                        item_kind: MethodFn,
                                                    },
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: True,
                                    pattern_ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)