Ok(
    [
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Type(
                TypeSynNodeDefn::Enum(
                    EnumTypeSynNodeDefn {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: EnumTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::result::Result`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 7,
                            template_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LaOrLtToken(
                                            TokenIdx(
                                                12,
                                            ),
                                        ),
                                        template_parameters: [
                                            TemplateParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: TemplateParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            TemplateParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: TemplateParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    14,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtToken(
                                            TokenIdx(
                                                16,
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
                                                        path: TypePath(`core::result::Result`, `Enum`),
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
                                                        14,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    13,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        16,
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
                                                                    15,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
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
            ImplBlockSynNodeDecl::TraitForType(
                TraitForTypeImplBlockSynNodeDecl {
                    syn_node_path: TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `core::result`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    ast_idx: 8,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            27,
                        ),
                    },
                    template_parameter_decl_list: Ok(
                        Some(
                            Generics {
                                langle: LaOrLtToken(
                                    TokenIdx(
                                        28,
                                    ),
                                ),
                                template_parameters: [
                                    TemplateParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `T1`,
                                                token_idx: TokenIdx(
                                                    29,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    TemplateParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `T2`,
                                                token_idx: TokenIdx(
                                                    31,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    TemplateParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 2,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `E1`,
                                                token_idx: TokenIdx(
                                                    33,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    TemplateParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 3,
                                        variant: TemplateParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `E2`,
                                                token_idx: TokenIdx(
                                                    35,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                ],
                                commas: [
                                    CommaToken(
                                        TokenIdx(
                                            30,
                                        ),
                                    ),
                                    CommaToken(
                                        TokenIdx(
                                            32,
                                        ),
                                    ),
                                    CommaToken(
                                        TokenIdx(
                                            34,
                                        ),
                                    ),
                                ],
                                decl_list_result: Ok(
                                    (),
                                ),
                                rangle: RaOrGtToken(
                                    TokenIdx(
                                        36,
                                    ),
                                ),
                            },
                        ),
                    ),
                    trai_expr: TraitExpr {
                        expr: 6,
                    },
                    for_token: ConnectionForToken {
                        token_idx: TokenIdx(
                            45,
                        ),
                    },
                    self_ty_decl: PathLeadingExpr(
                        SelfTypeExpr {
                            expr: 11,
                        },
                    ),
                    eol_colon: Ok(
                        EolToken::Colon(
                            EolColonToken {
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                        ),
                    ),
                    syn_expr_region: SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePath {
                                            path: TraitForTypeImplBlockPath {
                                                module_path: `core::result`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExpr::PrincipalEntityPath {
                                        item_path_expr: 2,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::ops::Unveil`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::PrincipalEntityPath {
                                        item_path_expr: 3,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 0,
                                        argument_expr_idx: 1,
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `T2`,
                                        token_idx: TokenIdx(
                                            43,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `T2`,
                                                    token_idx: TokenIdx(
                                                        31,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 2,
                                        argument_expr_idx: 3,
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `E2`,
                                        token_idx: TokenIdx(
                                            44,
                                        ),
                                        current_symbol_idx: 3,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `E2`,
                                                    token_idx: TokenIdx(
                                                        35,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 4,
                                        argument_expr_idx: 5,
                                    },
                                    SynExpr::PrincipalEntityPath {
                                        item_path_expr: 4,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `T1`,
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                        current_symbol_idx: 0,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `T1`,
                                                    token_idx: TokenIdx(
                                                        29,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 7,
                                        argument_expr_idx: 8,
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `E1`,
                                        token_idx: TokenIdx(
                                            48,
                                        ),
                                        current_symbol_idx: 2,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `E1`,
                                                    token_idx: TokenIdx(
                                                        33,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 9,
                                        argument_expr_idx: 10,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::CrateRoot(
                                            CrateToken {
                                                token_idx: TokenIdx(
                                                    37,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::Module(
                                            `core`,
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Subitem {
                                        parent: 0,
                                        scope_resolution_token: ScopeResolutionToken(
                                            TokenIdx(
                                                38,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentToken {
                                                ident: `ops`,
                                                token_idx: TokenIdx(
                                                    39,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::Module(
                                                `core::ops`,
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Subitem {
                                        parent: 1,
                                        scope_resolution_token: ScopeResolutionToken(
                                            TokenIdx(
                                                40,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentToken {
                                                ident: `Unveil`,
                                                token_idx: TokenIdx(
                                                    41,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::ops::Unveil`),
                                                ),
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `Result`,
                                                token_idx: TokenIdx(
                                                    42,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::result::Result`, `Enum`),
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `Result`,
                                                token_idx: TokenIdx(
                                                    46,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::result::Result`, `Enum`),
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
                                                30,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T1`,
                                                        token_idx: TokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSynSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                32,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T2`,
                                                        token_idx: TokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSynSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                34,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E1`,
                                                        token_idx: TokenIdx(
                                                            33,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSynSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                36,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E2`,
                                                        token_idx: TokenIdx(
                                                            35,
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
                                    (
                                        TemplateTypeParameter,
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                    (
                                        TemplateTypeParameter,
                                        ArenaIdxRange(
                                            2..3,
                                        ),
                                    ),
                                    (
                                        TemplateTypeParameter,
                                        ArenaIdxRange(
                                            3..4,
                                        ),
                                    ),
                                ],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: Trait,
                                    expr_idx: 6,
                                },
                                SynExprRoot {
                                    kind: SelfType,
                                    expr_idx: 11,
                                },
                            ],
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TraitForTypeItem(
                TraitForTypeItemSynNodeDefn::AssociatedType(
                    TraitForTypeAssociatedTypeSynNodeDefn {
                        syn_node_path: TraitForTypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Continue`,
                                    item_kind: AssociatedType,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TraitForTypeAssociatedTypeSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitForTypeItemPath {
                                        impl_block: TraitForTypeImplBlockPath {
                                            module_path: `core::result`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`core::result::Result`, `Enum`),
                                            ),
                                            disambiguator: 0,
                                        },
                                        ident: `Continue`,
                                        item_kind: AssociatedType,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            syn_node: TraitForTypeItemSynNode {
                                syn_node_path: TraitForTypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `core::result`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                            ident: `Continue`,
                                            item_kind: AssociatedType,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 3,
                                ident: `Continue`,
                                item_kind: AssociatedType,
                                visibility: Scope::PubUnder(
                                    `core::result`,
                                ),
                                is_generic: false,
                            },
                            ast_idx: 3,
                            generics: Ok(
                                None,
                            ),
                            eq_token: Ok(
                                EqToken(
                                    TokenIdx(
                                        52,
                                    ),
                                ),
                            ),
                            ty_term_expr_idx: 0,
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
                                                                    module_path: `core::result`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            item_path_expr: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::PrincipalEntityPath {
                                                            item_path_expr: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 0,
                                                            argument_expr_idx: 1,
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `T2`,
                                                            token_idx: TokenIdx(
                                                                43,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T2`,
                                                                        token_idx: TokenIdx(
                                                                            31,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `E2`,
                                                            token_idx: TokenIdx(
                                                                44,
                                                            ),
                                                            current_symbol_idx: 3,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E2`,
                                                                        token_idx: TokenIdx(
                                                                            35,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                        SynExpr::PrincipalEntityPath {
                                                            item_path_expr: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `T1`,
                                                            token_idx: TokenIdx(
                                                                47,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T1`,
                                                                        token_idx: TokenIdx(
                                                                            29,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 7,
                                                            argument_expr_idx: 8,
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `E1`,
                                                            token_idx: TokenIdx(
                                                                48,
                                                            ),
                                                            current_symbol_idx: 2,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E1`,
                                                                        token_idx: TokenIdx(
                                                                            33,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 9,
                                                            argument_expr_idx: 10,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::CrateRoot(
                                                                CrateToken {
                                                                    token_idx: TokenIdx(
                                                                        37,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    38,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `ops`,
                                                                    token_idx: TokenIdx(
                                                                        39,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    40,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `Unveil`,
                                                                    token_idx: TokenIdx(
                                                                        41,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `Result`,
                                                                    token_idx: TokenIdx(
                                                                        42,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `Result`,
                                                                    token_idx: TokenIdx(
                                                                        46,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
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
                                                                    30,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T1`,
                                                                            token_idx: TokenIdx(
                                                                                29,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    32,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T2`,
                                                                            token_idx: TokenIdx(
                                                                                31,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    34,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E1`,
                                                                            token_idx: TokenIdx(
                                                                                33,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    36,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E2`,
                                                                            token_idx: TokenIdx(
                                                                                35,
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
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: Trait,
                                                        expr_idx: 6,
                                                    },
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 11,
                                                    },
                                                ],
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
                                                                module_path: `core::result`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `Continue`,
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
                                            SynExpr::InheritedSymbol {
                                                ident: `E2`,
                                                token_idx: TokenIdx(
                                                    53,
                                                ),
                                                inherited_symbol_idx: 3,
                                                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
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
                                                            ident: `T1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `T2`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        3,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E2`,
                                                        },
                                                    ),
                                                },
                                            ],
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
                                                                ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                    TraitForTypeImplBlockSynNodePath {
                                                                        path: TraitForTypeImplBlockPath {
                                                                            module_path: `core::result`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: TypeSketch::Path(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                SynExpr::PrincipalEntityPath {
                                                                    item_path_expr: 2,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::MajorItem(
                                                                            MajorItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::PrincipalEntityPath {
                                                                    item_path_expr: 3,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::MajorItem(
                                                                            MajorItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 0,
                                                                    argument_expr_idx: 1,
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `T2`,
                                                                    token_idx: TokenIdx(
                                                                        43,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T2`,
                                                                                token_idx: TokenIdx(
                                                                                    31,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 2,
                                                                    argument_expr_idx: 3,
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `E2`,
                                                                    token_idx: TokenIdx(
                                                                        44,
                                                                    ),
                                                                    current_symbol_idx: 3,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E2`,
                                                                                token_idx: TokenIdx(
                                                                                    35,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 4,
                                                                    argument_expr_idx: 5,
                                                                },
                                                                SynExpr::PrincipalEntityPath {
                                                                    item_path_expr: 4,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::MajorItem(
                                                                            MajorItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `T1`,
                                                                    token_idx: TokenIdx(
                                                                        47,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T1`,
                                                                                token_idx: TokenIdx(
                                                                                    29,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 7,
                                                                    argument_expr_idx: 8,
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `E1`,
                                                                    token_idx: TokenIdx(
                                                                        48,
                                                                    ),
                                                                    current_symbol_idx: 2,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E1`,
                                                                                token_idx: TokenIdx(
                                                                                    33,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 9,
                                                                    argument_expr_idx: 10,
                                                                },
                                                            ],
                                                        },
                                                        principal_item_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::CrateRoot(
                                                                        CrateToken {
                                                                            token_idx: TokenIdx(
                                                                                37,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::Module(
                                                                        `core`,
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subitem {
                                                                    parent: 0,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            38,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `ops`,
                                                                            token_idx: TokenIdx(
                                                                                39,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::Module(
                                                                            `core::ops`,
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subitem {
                                                                    parent: 1,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            40,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `Unveil`,
                                                                            token_idx: TokenIdx(
                                                                                41,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::MajorItem(
                                                                            MajorItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `Result`,
                                                                            token_idx: TokenIdx(
                                                                                42,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `Result`,
                                                                            token_idx: TokenIdx(
                                                                                46,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
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
                                                                            30,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T1`,
                                                                                    token_idx: TokenIdx(
                                                                                        29,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            32,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T2`,
                                                                                    token_idx: TokenIdx(
                                                                                        31,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            34,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E1`,
                                                                                    token_idx: TokenIdx(
                                                                                        33,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            36,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E2`,
                                                                                    token_idx: TokenIdx(
                                                                                        35,
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
                                                                (
                                                                    TemplateTypeParameter,
                                                                    ArenaIdxRange(
                                                                        1..2,
                                                                    ),
                                                                ),
                                                                (
                                                                    TemplateTypeParameter,
                                                                    ArenaIdxRange(
                                                                        2..3,
                                                                    ),
                                                                ),
                                                                (
                                                                    TemplateTypeParameter,
                                                                    ArenaIdxRange(
                                                                        3..4,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            SynExprRoot {
                                                                kind: Trait,
                                                                expr_idx: 6,
                                                            },
                                                            SynExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 11,
                                                            },
                                                        ],
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
                                                                        module_path: `core::result`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `Continue`,
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
                                                    SynExpr::InheritedSymbol {
                                                        ident: `E2`,
                                                        token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                        inherited_symbol_idx: 3,
                                                        inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E2`,
                                                            },
                                                        ),
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
                                                                    ident: `T1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                1,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `T2`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                2,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `E1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                3,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `E2`,
                                                                },
                                                            ),
                                                        },
                                                    ],
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
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::AssociatedItem(
                                        AssociatedItemSynNodePath::TraitForTypeItem(
                                            TraitForTypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath {
                                                        impl_block: TraitForTypeImplBlockPath {
                                                            module_path: `core::result`,
                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                            ty_sketch: TypeSketch::Path(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `Continue`,
                                                        item_kind: AssociatedType,
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
                                                        ident: `T1`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `T2`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E1`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
                                            },
                                        ],
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
        SynNodeDefn::AssociatedItem(
            AssociatedItemSynNodeDefn::TraitForTypeItem(
                TraitForTypeItemSynNodeDefn::MethodFn(
                    TraitForTypeMethodFnSynNodeDefn {
                        syn_node_path: TraitForTypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `branch`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: TraitForTypeMethodFnSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitForTypeItemPath {
                                        impl_block: TraitForTypeImplBlockPath {
                                            module_path: `core::result`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`core::result::Result`, `Enum`),
                                            ),
                                            disambiguator: 0,
                                        },
                                        ident: `branch`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            node: TraitForTypeItemSynNode {
                                syn_node_path: TraitForTypeItemSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `core::result`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                            ident: `branch`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 4,
                                ident: `branch`,
                                item_kind: MethodFn,
                                visibility: Scope::PubUnder(
                                    `core::result`,
                                ),
                                is_generic: false,
                            },
                            ast_idx: 4,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                RitchieParameters {
                                    lpar: LparToken(
                                        TokenIdx(
                                            56,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        SpecificParameterDecl::Regular {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    58,
                                                ),
                                            ),
                                            ty: 4,
                                        },
                                    ],
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
                                    ReturnTypeExprBeforeColon {
                                        expr: 9,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            67,
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
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath {
                                                                path: TraitForTypeImplBlockPath {
                                                                    module_path: `core::result`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::PrincipalEntityPath {
                                                            item_path_expr: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::PrincipalEntityPath {
                                                            item_path_expr: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 0,
                                                            argument_expr_idx: 1,
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `T2`,
                                                            token_idx: TokenIdx(
                                                                43,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T2`,
                                                                        token_idx: TokenIdx(
                                                                            31,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `E2`,
                                                            token_idx: TokenIdx(
                                                                44,
                                                            ),
                                                            current_symbol_idx: 3,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E2`,
                                                                        token_idx: TokenIdx(
                                                                            35,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                        SynExpr::PrincipalEntityPath {
                                                            item_path_expr: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `T1`,
                                                            token_idx: TokenIdx(
                                                                47,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T1`,
                                                                        token_idx: TokenIdx(
                                                                            29,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 7,
                                                            argument_expr_idx: 8,
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `E1`,
                                                            token_idx: TokenIdx(
                                                                48,
                                                            ),
                                                            current_symbol_idx: 2,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E1`,
                                                                        token_idx: TokenIdx(
                                                                            33,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 9,
                                                            argument_expr_idx: 10,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::CrateRoot(
                                                                CrateToken {
                                                                    token_idx: TokenIdx(
                                                                        37,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    38,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `ops`,
                                                                    token_idx: TokenIdx(
                                                                        39,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    40,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `Unveil`,
                                                                    token_idx: TokenIdx(
                                                                        41,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `Result`,
                                                                    token_idx: TokenIdx(
                                                                        42,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `Result`,
                                                                    token_idx: TokenIdx(
                                                                        46,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
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
                                                                    30,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T1`,
                                                                            token_idx: TokenIdx(
                                                                                29,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    32,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T2`,
                                                                            token_idx: TokenIdx(
                                                                                31,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    34,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E1`,
                                                                            token_idx: TokenIdx(
                                                                                33,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    36,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E2`,
                                                                            token_idx: TokenIdx(
                                                                                35,
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
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: Trait,
                                                        expr_idx: 6,
                                                    },
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 11,
                                                    },
                                                ],
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
                                                                module_path: `core::result`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `branch`,
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
                                                            TypePath(`core::result::Result`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::InheritedSymbol {
                                                ident: `T2`,
                                                token_idx: TokenIdx(
                                                    60,
                                                ),
                                                inherited_symbol_idx: 1,
                                                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `T2`,
                                                    },
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            SynExpr::InheritedSymbol {
                                                ident: `E2`,
                                                token_idx: TokenIdx(
                                                    61,
                                                ),
                                                inherited_symbol_idx: 3,
                                                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::result::Result`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::InheritedSymbol {
                                                ident: `T1`,
                                                token_idx: TokenIdx(
                                                    65,
                                                ),
                                                inherited_symbol_idx: 0,
                                                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `T1`,
                                                    },
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExpr::InheritedSymbol {
                                                ident: `E1`,
                                                token_idx: TokenIdx(
                                                    66,
                                                ),
                                                inherited_symbol_idx: 2,
                                                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E1`,
                                                    },
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 7,
                                                argument_expr_idx: 8,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Result`,
                                                        token_idx: TokenIdx(
                                                            59,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Result`,
                                                        token_idx: TokenIdx(
                                                            64,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
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
                                                        ident: `result`,
                                                        token_idx: TokenIdx(
                                                            57,
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
                                                    `result`,
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
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `T1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `T2`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        3,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `E2`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        58,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `result`,
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
                                                    ty_expr_idx: 4,
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
                                            expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 9,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            1,
                        ),
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
                                                                ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                    TraitForTypeImplBlockSynNodePath {
                                                                        path: TraitForTypeImplBlockPath {
                                                                            module_path: `core::result`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: TypeSketch::Path(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                SynExpr::PrincipalEntityPath {
                                                                    item_path_expr: 2,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::MajorItem(
                                                                            MajorItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::PrincipalEntityPath {
                                                                    item_path_expr: 3,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::MajorItem(
                                                                            MajorItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 0,
                                                                    argument_expr_idx: 1,
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `T2`,
                                                                    token_idx: TokenIdx(
                                                                        43,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T2`,
                                                                                token_idx: TokenIdx(
                                                                                    31,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 2,
                                                                    argument_expr_idx: 3,
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `E2`,
                                                                    token_idx: TokenIdx(
                                                                        44,
                                                                    ),
                                                                    current_symbol_idx: 3,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E2`,
                                                                                token_idx: TokenIdx(
                                                                                    35,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 4,
                                                                    argument_expr_idx: 5,
                                                                },
                                                                SynExpr::PrincipalEntityPath {
                                                                    item_path_expr: 4,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::MajorItem(
                                                                            MajorItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `T1`,
                                                                    token_idx: TokenIdx(
                                                                        47,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T1`,
                                                                                token_idx: TokenIdx(
                                                                                    29,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 7,
                                                                    argument_expr_idx: 8,
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `E1`,
                                                                    token_idx: TokenIdx(
                                                                        48,
                                                                    ),
                                                                    current_symbol_idx: 2,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E1`,
                                                                                token_idx: TokenIdx(
                                                                                    33,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 9,
                                                                    argument_expr_idx: 10,
                                                                },
                                                            ],
                                                        },
                                                        principal_item_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::CrateRoot(
                                                                        CrateToken {
                                                                            token_idx: TokenIdx(
                                                                                37,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::Module(
                                                                        `core`,
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subitem {
                                                                    parent: 0,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            38,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `ops`,
                                                                            token_idx: TokenIdx(
                                                                                39,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::Module(
                                                                            `core::ops`,
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subitem {
                                                                    parent: 1,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            40,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `Unveil`,
                                                                            token_idx: TokenIdx(
                                                                                41,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::MajorItem(
                                                                            MajorItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `Result`,
                                                                            token_idx: TokenIdx(
                                                                                42,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `Result`,
                                                                            token_idx: TokenIdx(
                                                                                46,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
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
                                                                            30,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T1`,
                                                                                    token_idx: TokenIdx(
                                                                                        29,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            32,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T2`,
                                                                                    token_idx: TokenIdx(
                                                                                        31,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            34,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E1`,
                                                                                    token_idx: TokenIdx(
                                                                                        33,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            36,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                                syn_attrs: [],
                                                                            },
                                                                            annotated_variance_token: None,
                                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E2`,
                                                                                    token_idx: TokenIdx(
                                                                                        35,
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
                                                                (
                                                                    TemplateTypeParameter,
                                                                    ArenaIdxRange(
                                                                        1..2,
                                                                    ),
                                                                ),
                                                                (
                                                                    TemplateTypeParameter,
                                                                    ArenaIdxRange(
                                                                        2..3,
                                                                    ),
                                                                ),
                                                                (
                                                                    TemplateTypeParameter,
                                                                    ArenaIdxRange(
                                                                        3..4,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            SynExprRoot {
                                                                kind: Trait,
                                                                expr_idx: 6,
                                                            },
                                                            SynExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 11,
                                                            },
                                                        ],
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
                                                                        module_path: `core::result`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `branch`,
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
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::InheritedSymbol {
                                                        ident: `T2`,
                                                        token_idx: TokenIdx(
                                                            60,
                                                        ),
                                                        inherited_symbol_idx: 1,
                                                        inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `T2`,
                                                            },
                                                        ),
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 0,
                                                        argument_expr_idx: 1,
                                                    },
                                                    SynExpr::InheritedSymbol {
                                                        ident: `E2`,
                                                        token_idx: TokenIdx(
                                                            61,
                                                        ),
                                                        inherited_symbol_idx: 3,
                                                        inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E2`,
                                                            },
                                                        ),
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 2,
                                                        argument_expr_idx: 3,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::InheritedSymbol {
                                                        ident: `T1`,
                                                        token_idx: TokenIdx(
                                                            65,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `T1`,
                                                            },
                                                        ),
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 5,
                                                        argument_expr_idx: 6,
                                                    },
                                                    SynExpr::InheritedSymbol {
                                                        ident: `E1`,
                                                        token_idx: TokenIdx(
                                                            66,
                                                        ),
                                                        inherited_symbol_idx: 2,
                                                        inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSynSymbol::Type {
                                                                ident: `E1`,
                                                            },
                                                        ),
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 7,
                                                        argument_expr_idx: 8,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Result`,
                                                                token_idx: TokenIdx(
                                                                    59,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Result`,
                                                                token_idx: TokenIdx(
                                                                    64,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::result::Result`, `Enum`),
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
                                                                ident: `result`,
                                                                token_idx: TokenIdx(
                                                                    57,
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
                                                            `result`,
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
                                                    data: [
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `T1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                1,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `T2`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                2,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `E1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                3,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `E2`,
                                                                },
                                                            ),
                                                        },
                                                    ],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                58,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `result`,
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
                                                            ty_expr_idx: 4,
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
                                                    expr_idx: 4,
                                                },
                                                SynExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 9,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::AssociatedItem(
                                        AssociatedItemSynNodePath::TraitForTypeItem(
                                            TraitForTypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath {
                                                        impl_block: TraitForTypeImplBlockPath {
                                                            module_path: `core::result`,
                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                            ty_sketch: TypeSketch::Path(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `branch`,
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
                                        SynExpr::Todo {
                                            token_idx: TokenIdx(
                                                68,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Eval {
                                            expr_idx: 0,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                    ],
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
                                                        ident: `T1`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `T2`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E1`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ExplicitParameter {
                                                    ident: `result`,
                                                },
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
                                        kind: EvalExpr,
                                        expr_idx: 0,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 1,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)