Ok(
    [
        NodeDefn::ModuleItem(
            ModuleItemNodeDefn::Type(
                TypeNodeDefn::Enum(
                    EnumTypeNodeDefn {
                        node_path: TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: EnumTypeNodeDecl {
                            node_path: TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::result::Result`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 6,
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                8,
                                            ),
                                        ),
                                        generic_parameters: [
                                            GenericParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: GenericParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            GenericParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: GenericParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    10,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                12,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Type(
                                                TypeNodePath {
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
                                    principal_entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
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
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    9,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    11,
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
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ImplicitTypeParameter,
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
        NodeDefn::ImplBlock(
            ImplBlockNodeDecl::TraitForType(
                TraitForTypeImplBlockNodeDecl {
                    node_path: TraitForTypeImplBlockNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `core::result`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: Path(
                                TypePath(
                                    Id {
                                        value: 31,
                                    },
                                ),
                            ),
                            disambiguator: 0,
                        },
                    },
                    ast_idx: 7,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            23,
                        ),
                    },
                    implicit_parameter_decl_list: Ok(
                        Some(
                            Generics {
                                langle: LeftAngleBracketOrLessThanToken(
                                    TokenIdx(
                                        24,
                                    ),
                                ),
                                generic_parameters: [
                                    GenericParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: GenericParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `T1`,
                                                token_idx: TokenIdx(
                                                    25,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    GenericParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        variant: GenericParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `T2`,
                                                token_idx: TokenIdx(
                                                    27,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    GenericParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 2,
                                        variant: GenericParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `E1`,
                                                token_idx: TokenIdx(
                                                    29,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    GenericParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 3,
                                        variant: GenericParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `E2`,
                                                token_idx: TokenIdx(
                                                    31,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                ],
                                commas: [
                                    CommaToken(
                                        TokenIdx(
                                            26,
                                        ),
                                    ),
                                    CommaToken(
                                        TokenIdx(
                                            28,
                                        ),
                                    ),
                                    CommaToken(
                                        TokenIdx(
                                            30,
                                        ),
                                    ),
                                ],
                                decl_list_result: Ok(
                                    (),
                                ),
                                rangle: RightAngleBracketToken(
                                    TokenIdx(
                                        32,
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
                            41,
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
                                    45,
                                ),
                            },
                        ),
                    ),
                    expr_region: ExprRegion {
                        data: ExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                EntityNodePath::ImplBlock(
                                    ImplBlockNodePath::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockNodePath {
                                            path: TraitForTypeImplBlockPath {
                                                module_path: `core::result`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: Path(
                                                    TypePath(
                                                        Id {
                                                            value: 31,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    Expr::PrincipalEntityPath {
                                        entity_path_expr: 2,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::Unveil`),
                                                ),
                                            ),
                                        ),
                                    },
                                    Expr::PrincipalEntityPath {
                                        entity_path_expr: 3,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    Expr::ExplicitApplication {
                                        function_expr_idx: 0,
                                        argument_expr_idx: 1,
                                    },
                                    Expr::CurrentSymbol {
                                        ident: `T2`,
                                        token_idx: TokenIdx(
                                            39,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `T2`,
                                                    token_idx: TokenIdx(
                                                        27,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    Expr::ExplicitApplication {
                                        function_expr_idx: 2,
                                        argument_expr_idx: 3,
                                    },
                                    Expr::CurrentSymbol {
                                        ident: `E2`,
                                        token_idx: TokenIdx(
                                            40,
                                        ),
                                        current_symbol_idx: 3,
                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `E2`,
                                                    token_idx: TokenIdx(
                                                        31,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    Expr::ExplicitApplication {
                                        function_expr_idx: 4,
                                        argument_expr_idx: 5,
                                    },
                                    Expr::PrincipalEntityPath {
                                        entity_path_expr: 4,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    Expr::CurrentSymbol {
                                        ident: `T1`,
                                        token_idx: TokenIdx(
                                            43,
                                        ),
                                        current_symbol_idx: 0,
                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `T1`,
                                                    token_idx: TokenIdx(
                                                        25,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    Expr::ExplicitApplication {
                                        function_expr_idx: 7,
                                        argument_expr_idx: 8,
                                    },
                                    Expr::CurrentSymbol {
                                        ident: `E1`,
                                        token_idx: TokenIdx(
                                            44,
                                        ),
                                        current_symbol_idx: 2,
                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `E1`,
                                                    token_idx: TokenIdx(
                                                        29,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    Expr::ExplicitApplication {
                                        function_expr_idx: 9,
                                        argument_expr_idx: 10,
                                    },
                                ],
                            },
                            principal_entity_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::CrateRoot(
                                            CrateToken {
                                                token_idx: TokenIdx(
                                                    33,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::Module(
                                            `core`,
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Subentity {
                                        parent: 0,
                                        scope_resolution_token: ScopeResolutionToken(
                                            TokenIdx(
                                                34,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentToken {
                                                ident: `ops`,
                                                token_idx: TokenIdx(
                                                    35,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::Module(
                                                `core::ops`,
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Subentity {
                                        parent: 1,
                                        scope_resolution_token: ScopeResolutionToken(
                                            TokenIdx(
                                                36,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentToken {
                                                ident: `Unveil`,
                                                token_idx: TokenIdx(
                                                    37,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
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
                                                    38,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`core::result::Result`, `Enum`),
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
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`core::result::Result`, `Enum`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: PatternExprRegion {
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
                            symbol_region: SymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [
                                        CurrentSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                26,
                                            ),
                                            access_end: None,
                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T1`,
                                                        token_idx: TokenIdx(
                                                            25,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                28,
                                            ),
                                            access_end: None,
                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T2`,
                                                        token_idx: TokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                30,
                                            ),
                                            access_end: None,
                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E1`,
                                                        token_idx: TokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                32,
                                            ),
                                            access_end: None,
                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E2`,
                                                        token_idx: TokenIdx(
                                                            31,
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
                                        ImplicitTypeParameter,
                                        ArenaIdxRange(
                                            0..1,
                                        ),
                                    ),
                                    (
                                        ImplicitTypeParameter,
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                    (
                                        ImplicitTypeParameter,
                                        ArenaIdxRange(
                                            2..3,
                                        ),
                                    ),
                                    (
                                        ImplicitTypeParameter,
                                        ArenaIdxRange(
                                            3..4,
                                        ),
                                    ),
                                ],
                            },
                            roots: [
                                ExprRoot {
                                    kind: Trait,
                                    expr_idx: 6,
                                },
                                ExprRoot {
                                    kind: SelfType,
                                    expr_idx: 11,
                                },
                            ],
                        },
                    },
                },
            ),
        ),
        NodeDefn::AssociatedItem(
            AssociatedItemNodeDefn::TraitForTypeItem(
                TraitForTypeItemNodeDefn::AssociatedType(
                    TraitForTypeAssociatedTypeNodeDefn {
                        node_path: TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: Path(
                                            TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Continue`,
                                    item_kind: AssociatedType,
                                },
                                disambiguator: 0,
                            },
                        },
                        node_decl: TraitForTypeAssociatedTypeNodeDecl {
                            node_path: TraitForTypeItemNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitForTypeItemPath {
                                        impl_block: TraitForTypeImplBlockPath {
                                            module_path: `core::result`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: Path(
                                                TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                        ident: `Continue`,
                                        item_kind: AssociatedType,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            node: TraitForTypeItemNode {
                                node_path: TraitForTypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `core::result`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: Path(
                                                    TypePath(
                                                        Id {
                                                            value: 31,
                                                        },
                                                    ),
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
                                        48,
                                    ),
                                ),
                            ),
                            ty_term_expr_idx: 0,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockNodePath {
                                                                path: TraitForTypeImplBlockPath {
                                                                    module_path: `core::result`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: Path(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 0,
                                                            argument_expr_idx: 1,
                                                        },
                                                        Expr::CurrentSymbol {
                                                            ident: `T2`,
                                                            token_idx: TokenIdx(
                                                                39,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T2`,
                                                                        token_idx: TokenIdx(
                                                                            27,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        Expr::CurrentSymbol {
                                                            ident: `E2`,
                                                            token_idx: TokenIdx(
                                                                40,
                                                            ),
                                                            current_symbol_idx: 3,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E2`,
                                                                        token_idx: TokenIdx(
                                                                            31,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::CurrentSymbol {
                                                            ident: `T1`,
                                                            token_idx: TokenIdx(
                                                                43,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T1`,
                                                                        token_idx: TokenIdx(
                                                                            25,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 7,
                                                            argument_expr_idx: 8,
                                                        },
                                                        Expr::CurrentSymbol {
                                                            ident: `E1`,
                                                            token_idx: TokenIdx(
                                                                44,
                                                            ),
                                                            current_symbol_idx: 2,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E1`,
                                                                        token_idx: TokenIdx(
                                                                            29,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 9,
                                                            argument_expr_idx: 10,
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::CrateRoot(
                                                                CrateToken {
                                                                    token_idx: TokenIdx(
                                                                        33,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subentity {
                                                            parent: 0,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    34,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `ops`,
                                                                    token_idx: TokenIdx(
                                                                        35,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subentity {
                                                            parent: 1,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    36,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `Unveil`,
                                                                    token_idx: TokenIdx(
                                                                        37,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
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
                                                                        38,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
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
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: PatternExprRegion {
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
                                                symbol_region: SymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    26,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T1`,
                                                                            token_idx: TokenIdx(
                                                                                25,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    28,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T2`,
                                                                            token_idx: TokenIdx(
                                                                                27,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    30,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E1`,
                                                                            token_idx: TokenIdx(
                                                                                29,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    32,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E2`,
                                                                            token_idx: TokenIdx(
                                                                                31,
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
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                        (
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                        (
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                roots: [
                                                    ExprRoot {
                                                        kind: Trait,
                                                        expr_idx: 6,
                                                    },
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 11,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TraitForTypeItem(
                                                TraitForTypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TraitForTypeItemPath {
                                                            impl_block: TraitForTypeImplBlockPath {
                                                                module_path: `core::result`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: Path(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
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
                                            Expr::InheritedSymbol {
                                                ident: `E2`,
                                                token_idx: TokenIdx(
                                                    49,
                                                ),
                                                inherited_symbol_idx: 3,
                                                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
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
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [
                                                InheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSymbol::Type {
                                                            ident: `T1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSymbol::Type {
                                                            ident: `T2`,
                                                        },
                                                    ),
                                                },
                                                InheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSymbol::Type {
                                                            ident: `E1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        3,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSymbol::Type {
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
                                        ExprRoot {
                                            kind: AssociatedTypeTerm,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: None,
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TraitForTypeImplBlock(
                                                                    TraitForTypeImplBlockNodePath {
                                                                        path: TraitForTypeImplBlockPath {
                                                                            module_path: `core::result`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: Path(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 31,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 2,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 3,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 0,
                                                                    argument_expr_idx: 1,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `T2`,
                                                                    token_idx: TokenIdx(
                                                                        39,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T2`,
                                                                                token_idx: TokenIdx(
                                                                                    27,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 2,
                                                                    argument_expr_idx: 3,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `E2`,
                                                                    token_idx: TokenIdx(
                                                                        40,
                                                                    ),
                                                                    current_symbol_idx: 3,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E2`,
                                                                                token_idx: TokenIdx(
                                                                                    31,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 4,
                                                                    argument_expr_idx: 5,
                                                                },
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 4,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `T1`,
                                                                    token_idx: TokenIdx(
                                                                        43,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T1`,
                                                                                token_idx: TokenIdx(
                                                                                    25,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 7,
                                                                    argument_expr_idx: 8,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `E1`,
                                                                    token_idx: TokenIdx(
                                                                        44,
                                                                    ),
                                                                    current_symbol_idx: 2,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E1`,
                                                                                token_idx: TokenIdx(
                                                                                    29,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 9,
                                                                    argument_expr_idx: 10,
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::CrateRoot(
                                                                        CrateToken {
                                                                            token_idx: TokenIdx(
                                                                                33,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::Module(
                                                                        `core`,
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subentity {
                                                                    parent: 0,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            34,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `ops`,
                                                                            token_idx: TokenIdx(
                                                                                35,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::Module(
                                                                            `core::ops`,
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subentity {
                                                                    parent: 1,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            36,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `Unveil`,
                                                                            token_idx: TokenIdx(
                                                                                37,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
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
                                                                                38,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
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
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        stmt_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_region: PatternExprRegion {
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
                                                        symbol_region: SymbolRegion {
                                                            inherited_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            current_symbol_arena: Arena {
                                                                data: [
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            26,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T1`,
                                                                                    token_idx: TokenIdx(
                                                                                        25,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            28,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T2`,
                                                                                    token_idx: TokenIdx(
                                                                                        27,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            30,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E1`,
                                                                                    token_idx: TokenIdx(
                                                                                        29,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            32,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E2`,
                                                                                    token_idx: TokenIdx(
                                                                                        31,
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
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        0..1,
                                                                    ),
                                                                ),
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        1..2,
                                                                    ),
                                                                ),
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        2..3,
                                                                    ),
                                                                ),
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        3..4,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            ExprRoot {
                                                                kind: Trait,
                                                                expr_idx: 6,
                                                            },
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 11,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TraitForTypeItem(
                                                        TraitForTypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitForTypeItemPath {
                                                                    impl_block: TraitForTypeImplBlockPath {
                                                                        module_path: `core::result`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: Path(
                                                                            TypePath(
                                                                                Id {
                                                                                    value: 31,
                                                                                },
                                                                            ),
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
                                                    Expr::InheritedSymbol {
                                                        ident: `E2`,
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                        inherited_symbol_idx: 3,
                                                        inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `E2`,
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
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
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `T1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                1,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `T2`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                2,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                3,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
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
                                                ExprRoot {
                                                    kind: AssociatedTypeTerm,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TraitForTypeItem(
                                            TraitForTypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath {
                                                        impl_block: TraitForTypeImplBlockPath {
                                                            module_path: `core::result`,
                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                            ty_sketch: Path(
                                                                TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
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
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternExprRegion {
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
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `T1`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `T2`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `E1`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
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
        NodeDefn::AssociatedItem(
            AssociatedItemNodeDefn::TraitForTypeItem(
                TraitForTypeItemNodeDefn::MethodFn(
                    TraitForTypeMethodFnNodeDefn {
                        node_path: TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: Path(
                                            TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `branch`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        node_decl: TraitForTypeMethodFnNodeDecl {
                            node_path: TraitForTypeItemNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitForTypeItemPath {
                                        impl_block: TraitForTypeImplBlockPath {
                                            module_path: `core::result`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: Path(
                                                TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                        ident: `branch`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            node: TraitForTypeItemNode {
                                node_path: TraitForTypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `core::result`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: Path(
                                                    TypePath(
                                                        Id {
                                                            value: 31,
                                                        },
                                                    ),
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
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            parenic_parameter_decl_list: Ok(
                                SelfParameterAndExplicitParameters {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            52,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenic_parameters: [
                                        SpecificParameterDecl::Regular {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    54,
                                                ),
                                            ),
                                            ty: 4,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            58,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            59,
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
                                            63,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockNodePath {
                                                                path: TraitForTypeImplBlockPath {
                                                                    module_path: `core::result`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: Path(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 0,
                                                            argument_expr_idx: 1,
                                                        },
                                                        Expr::CurrentSymbol {
                                                            ident: `T2`,
                                                            token_idx: TokenIdx(
                                                                39,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T2`,
                                                                        token_idx: TokenIdx(
                                                                            27,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        Expr::CurrentSymbol {
                                                            ident: `E2`,
                                                            token_idx: TokenIdx(
                                                                40,
                                                            ),
                                                            current_symbol_idx: 3,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E2`,
                                                                        token_idx: TokenIdx(
                                                                            31,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        Expr::CurrentSymbol {
                                                            ident: `T1`,
                                                            token_idx: TokenIdx(
                                                                43,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T1`,
                                                                        token_idx: TokenIdx(
                                                                            25,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 7,
                                                            argument_expr_idx: 8,
                                                        },
                                                        Expr::CurrentSymbol {
                                                            ident: `E1`,
                                                            token_idx: TokenIdx(
                                                                44,
                                                            ),
                                                            current_symbol_idx: 2,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E1`,
                                                                        token_idx: TokenIdx(
                                                                            29,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        Expr::ExplicitApplication {
                                                            function_expr_idx: 9,
                                                            argument_expr_idx: 10,
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::CrateRoot(
                                                                CrateToken {
                                                                    token_idx: TokenIdx(
                                                                        33,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subentity {
                                                            parent: 0,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    34,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `ops`,
                                                                    token_idx: TokenIdx(
                                                                        35,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subentity {
                                                            parent: 1,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    36,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `Unveil`,
                                                                    token_idx: TokenIdx(
                                                                        37,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
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
                                                                        38,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
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
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: PatternExprRegion {
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
                                                symbol_region: SymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    26,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T1`,
                                                                            token_idx: TokenIdx(
                                                                                25,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    28,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T2`,
                                                                            token_idx: TokenIdx(
                                                                                27,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    30,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E1`,
                                                                            token_idx: TokenIdx(
                                                                                29,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    32,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E2`,
                                                                            token_idx: TokenIdx(
                                                                                31,
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
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                        (
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                        (
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                roots: [
                                                    ExprRoot {
                                                        kind: Trait,
                                                        expr_idx: 6,
                                                    },
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 11,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TraitForTypeItem(
                                                TraitForTypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TraitForTypeItemPath {
                                                            impl_block: TraitForTypeImplBlockPath {
                                                                module_path: `core::result`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: Path(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
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
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::result::Result`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::InheritedSymbol {
                                                ident: `T2`,
                                                token_idx: TokenIdx(
                                                    56,
                                                ),
                                                inherited_symbol_idx: 1,
                                                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `T2`,
                                                    },
                                                ),
                                            },
                                            Expr::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            Expr::InheritedSymbol {
                                                ident: `E2`,
                                                token_idx: TokenIdx(
                                                    57,
                                                ),
                                                inherited_symbol_idx: 3,
                                                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
                                            },
                                            Expr::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::result::Result`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::InheritedSymbol {
                                                ident: `T1`,
                                                token_idx: TokenIdx(
                                                    61,
                                                ),
                                                inherited_symbol_idx: 0,
                                                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `T1`,
                                                    },
                                                ),
                                            },
                                            Expr::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            Expr::InheritedSymbol {
                                                ident: `E1`,
                                                token_idx: TokenIdx(
                                                    62,
                                                ),
                                                inherited_symbol_idx: 2,
                                                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `E1`,
                                                    },
                                                ),
                                            },
                                            Expr::ExplicitApplication {
                                                function_expr_idx: 7,
                                                argument_expr_idx: 8,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Result`,
                                                        token_idx: TokenIdx(
                                                            55,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Result`,
                                                        token_idx: TokenIdx(
                                                            60,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `result`,
                                                        token_idx: TokenIdx(
                                                            53,
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
                                                PatternSymbol::Atom(
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
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [
                                                InheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSymbol::Type {
                                                            ident: `T1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSymbol::Type {
                                                            ident: `T2`,
                                                        },
                                                    ),
                                                },
                                                InheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSymbol::Type {
                                                            ident: `E1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        3,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSymbol::Type {
                                                            ident: `E2`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        54,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 4,
                                        },
                                        ExprRoot {
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
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TraitForTypeImplBlock(
                                                                    TraitForTypeImplBlockNodePath {
                                                                        path: TraitForTypeImplBlockPath {
                                                                            module_path: `core::result`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: Path(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 31,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 2,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 3,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 0,
                                                                    argument_expr_idx: 1,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `T2`,
                                                                    token_idx: TokenIdx(
                                                                        39,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T2`,
                                                                                token_idx: TokenIdx(
                                                                                    27,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 2,
                                                                    argument_expr_idx: 3,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `E2`,
                                                                    token_idx: TokenIdx(
                                                                        40,
                                                                    ),
                                                                    current_symbol_idx: 3,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E2`,
                                                                                token_idx: TokenIdx(
                                                                                    31,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 4,
                                                                    argument_expr_idx: 5,
                                                                },
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 4,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `T1`,
                                                                    token_idx: TokenIdx(
                                                                        43,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T1`,
                                                                                token_idx: TokenIdx(
                                                                                    25,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 7,
                                                                    argument_expr_idx: 8,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `E1`,
                                                                    token_idx: TokenIdx(
                                                                        44,
                                                                    ),
                                                                    current_symbol_idx: 2,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E1`,
                                                                                token_idx: TokenIdx(
                                                                                    29,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function_expr_idx: 9,
                                                                    argument_expr_idx: 10,
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::CrateRoot(
                                                                        CrateToken {
                                                                            token_idx: TokenIdx(
                                                                                33,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::Module(
                                                                        `core`,
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subentity {
                                                                    parent: 0,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            34,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `ops`,
                                                                            token_idx: TokenIdx(
                                                                                35,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::Module(
                                                                            `core::ops`,
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subentity {
                                                                    parent: 1,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            36,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `Unveil`,
                                                                            token_idx: TokenIdx(
                                                                                37,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
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
                                                                                38,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
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
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        stmt_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_region: PatternExprRegion {
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
                                                        symbol_region: SymbolRegion {
                                                            inherited_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            current_symbol_arena: Arena {
                                                                data: [
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            26,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T1`,
                                                                                    token_idx: TokenIdx(
                                                                                        25,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            28,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T2`,
                                                                                    token_idx: TokenIdx(
                                                                                        27,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            30,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E1`,
                                                                                    token_idx: TokenIdx(
                                                                                        29,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            32,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E2`,
                                                                                    token_idx: TokenIdx(
                                                                                        31,
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
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        0..1,
                                                                    ),
                                                                ),
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        1..2,
                                                                    ),
                                                                ),
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        2..3,
                                                                    ),
                                                                ),
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        3..4,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            ExprRoot {
                                                                kind: Trait,
                                                                expr_idx: 6,
                                                            },
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 11,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TraitForTypeItem(
                                                        TraitForTypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitForTypeItemPath {
                                                                    impl_block: TraitForTypeImplBlockPath {
                                                                        module_path: `core::result`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: Path(
                                                                            TypePath(
                                                                                Id {
                                                                                    value: 31,
                                                                                },
                                                                            ),
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
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `T2`,
                                                        token_idx: TokenIdx(
                                                            56,
                                                        ),
                                                        inherited_symbol_idx: 1,
                                                        inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `T2`,
                                                            },
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function_expr_idx: 0,
                                                        argument_expr_idx: 1,
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `E2`,
                                                        token_idx: TokenIdx(
                                                            57,
                                                        ),
                                                        inherited_symbol_idx: 3,
                                                        inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `E2`,
                                                            },
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function_expr_idx: 2,
                                                        argument_expr_idx: 3,
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `T1`,
                                                        token_idx: TokenIdx(
                                                            61,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `T1`,
                                                            },
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function_expr_idx: 5,
                                                        argument_expr_idx: 6,
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `E1`,
                                                        token_idx: TokenIdx(
                                                            62,
                                                        ),
                                                        inherited_symbol_idx: 2,
                                                        inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `E1`,
                                                            },
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function_expr_idx: 7,
                                                        argument_expr_idx: 8,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Result`,
                                                                token_idx: TokenIdx(
                                                                    55,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Result`,
                                                                token_idx: TokenIdx(
                                                                    60,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `result`,
                                                                token_idx: TokenIdx(
                                                                    53,
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
                                                        PatternSymbol::Atom(
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
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `T1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                1,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `T2`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                2,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                3,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E2`,
                                                                },
                                                            ),
                                                        },
                                                    ],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                54,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 9,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TraitForTypeItem(
                                            TraitForTypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath {
                                                        impl_block: TraitForTypeImplBlockPath {
                                                            module_path: `core::result`,
                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                            ty_sketch: Path(
                                                                TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
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
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        64,
                                                    ),
                                                    ident: `todo`,
                                                },
                                            ),
                                        ),
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
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
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `T1`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `T2`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `E1`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
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
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 0,
                                    },
                                    ExprRoot {
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