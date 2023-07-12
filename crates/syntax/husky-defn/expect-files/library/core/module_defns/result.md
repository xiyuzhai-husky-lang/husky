Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Enum(
                    EnumTypeDefn {
                        path: TypePath(`core::result::Result`, `Enum`),
                        decl: EnumTypeDecl {
                            path: TypePath(`core::result::Result`, `Enum`),
                            implicit_parameters: [
                                ImplicitParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `T`,
                                            token_idx: TokenIdx(
                                                4,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                ImplicitParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `E`,
                                            token_idx: TokenIdx(
                                                6,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                            ],
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
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        7,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    6,
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
        Defn::ImplBlock(
            ImplBlockDecl::TraitForType(
                TraitForTypeImplBlockDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `core::result`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_path: TypePath(`core::result::Result`, `Enum`),
                        disambiguator: 0,
                    },
                    implicit_parameters: [
                        ImplicitParameterDecl {
                            annotated_variance_token: None,
                            symbol: 0,
                            variant: ImplicitParameterDeclPatternVariant::Type {
                                ident_token: IdentToken {
                                    ident: `T1`,
                                    token_idx: TokenIdx(
                                        20,
                                    ),
                                },
                                traits: None,
                            },
                        },
                        ImplicitParameterDecl {
                            annotated_variance_token: None,
                            symbol: 1,
                            variant: ImplicitParameterDeclPatternVariant::Type {
                                ident_token: IdentToken {
                                    ident: `T2`,
                                    token_idx: TokenIdx(
                                        22,
                                    ),
                                },
                                traits: None,
                            },
                        },
                        ImplicitParameterDecl {
                            annotated_variance_token: None,
                            symbol: 2,
                            variant: ImplicitParameterDeclPatternVariant::Type {
                                ident_token: IdentToken {
                                    ident: `E1`,
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                                traits: None,
                            },
                        },
                        ImplicitParameterDecl {
                            annotated_variance_token: None,
                            symbol: 3,
                            variant: ImplicitParameterDeclPatternVariant::Type {
                                ident_token: IdentToken {
                                    ident: `E2`,
                                    token_idx: TokenIdx(
                                        26,
                                    ),
                                },
                                traits: None,
                            },
                        },
                    ],
                    trai_expr: TraitExpr {
                        expr: 6,
                    },
                    ty_expr: TypeExpr {
                        expr: 11,
                    },
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
                                                ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                            34,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `T2`,
                                                    token_idx: TokenIdx(
                                                        22,
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
                                            35,
                                        ),
                                        current_symbol_idx: 3,
                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `E2`,
                                                    token_idx: TokenIdx(
                                                        26,
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
                                            38,
                                        ),
                                        current_symbol_idx: 0,
                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `T1`,
                                                    token_idx: TokenIdx(
                                                        20,
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
                                            39,
                                        ),
                                        current_symbol_idx: 2,
                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `E1`,
                                                    token_idx: TokenIdx(
                                                        24,
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
                                                    28,
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
                                                29,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentToken {
                                                ident: `ops`,
                                                token_idx: TokenIdx(
                                                    30,
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
                                                31,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentToken {
                                                ident: `Unveil`,
                                                token_idx: TokenIdx(
                                                    32,
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
                                                    33,
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
                                                    37,
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
                                                21,
                                            ),
                                            access_end: None,
                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T1`,
                                                        token_idx: TokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                23,
                                            ),
                                            access_end: None,
                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T2`,
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                25,
                                            ),
                                            access_end: None,
                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E1`,
                                                        token_idx: TokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                27,
                                            ),
                                            access_end: None,
                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E2`,
                                                        token_idx: TokenIdx(
                                                            26,
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
        Defn::AssociatedItem(
            AssociatedItemDefn::TraitForTypeItem(
                TraitForTypeItemDefn::AssociatedType(
                    TraitForTypeAssociatedTypeDefn {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `core::result`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                            ident: `Continue`,
                            item_kind: AssociatedType,
                        },
                        decl: TraitForTypeAssociatedTypeDecl {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `core::result`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_path: TypePath(`core::result::Result`, `Enum`),
                                    disambiguator: 0,
                                },
                                ident: `Continue`,
                                item_kind: AssociatedType,
                            },
                            implicit_parameters: [],
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
                                                                    ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                                                34,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T2`,
                                                                        token_idx: TokenIdx(
                                                                            22,
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
                                                                35,
                                                            ),
                                                            current_symbol_idx: 3,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E2`,
                                                                        token_idx: TokenIdx(
                                                                            26,
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
                                                                38,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T1`,
                                                                        token_idx: TokenIdx(
                                                                            20,
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
                                                                39,
                                                            ),
                                                            current_symbol_idx: 2,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E1`,
                                                                        token_idx: TokenIdx(
                                                                            24,
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
                                                                        28,
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
                                                                    29,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `ops`,
                                                                    token_idx: TokenIdx(
                                                                        30,
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
                                                                    31,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `Unveil`,
                                                                    token_idx: TokenIdx(
                                                                        32,
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
                                                                        33,
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
                                                                        37,
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
                                                                    21,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T1`,
                                                                            token_idx: TokenIdx(
                                                                                20,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    23,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T2`,
                                                                            token_idx: TokenIdx(
                                                                                22,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    25,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E1`,
                                                                            token_idx: TokenIdx(
                                                                                24,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    27,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E2`,
                                                                            token_idx: TokenIdx(
                                                                                26,
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
                                                                ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                                                            ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                                                        34,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T2`,
                                                                                token_idx: TokenIdx(
                                                                                    22,
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
                                                                        35,
                                                                    ),
                                                                    current_symbol_idx: 3,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E2`,
                                                                                token_idx: TokenIdx(
                                                                                    26,
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
                                                                        38,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T1`,
                                                                                token_idx: TokenIdx(
                                                                                    20,
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
                                                                        39,
                                                                    ),
                                                                    current_symbol_idx: 2,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E1`,
                                                                                token_idx: TokenIdx(
                                                                                    24,
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
                                                                                28,
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
                                                                            29,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `ops`,
                                                                            token_idx: TokenIdx(
                                                                                30,
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
                                                                            31,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `Unveil`,
                                                                            token_idx: TokenIdx(
                                                                                32,
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
                                                                                33,
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
                                                                                37,
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
                                                                            21,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T1`,
                                                                                    token_idx: TokenIdx(
                                                                                        20,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            23,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T2`,
                                                                                    token_idx: TokenIdx(
                                                                                        22,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            25,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E1`,
                                                                                    token_idx: TokenIdx(
                                                                                        24,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            27,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E2`,
                                                                                    token_idx: TokenIdx(
                                                                                        26,
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
                                                                        ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                                            ty_path: TypePath(`core::result::Result`, `Enum`),
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
        Defn::AssociatedItem(
            AssociatedItemDefn::TraitForTypeItem(
                TraitForTypeItemDefn::MethodFn(
                    TraitForTypeMethodFnDefn {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `core::result`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                            ident: `branch`,
                            item_kind: MethodFn,
                        },
                        decl: TraitForTypeMethodFnDecl {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `core::result`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_path: TypePath(`core::result::Result`, `Enum`),
                                    disambiguator: 0,
                                },
                                ident: `branch`,
                                item_kind: MethodFn,
                            },
                            implicit_parameters: [],
                            self_parameter: None,
                            explicit_parameters: [
                                ExplicitParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            49,
                                        ),
                                    ),
                                    ty: 4,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExprBeforeColon {
                                    expr: 9,
                                },
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
                                                                    ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                                                34,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T2`,
                                                                        token_idx: TokenIdx(
                                                                            22,
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
                                                                35,
                                                            ),
                                                            current_symbol_idx: 3,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E2`,
                                                                        token_idx: TokenIdx(
                                                                            26,
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
                                                                38,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T1`,
                                                                        token_idx: TokenIdx(
                                                                            20,
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
                                                                39,
                                                            ),
                                                            current_symbol_idx: 2,
                                                            current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E1`,
                                                                        token_idx: TokenIdx(
                                                                            24,
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
                                                                        28,
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
                                                                    29,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `ops`,
                                                                    token_idx: TokenIdx(
                                                                        30,
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
                                                                    31,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `Unveil`,
                                                                    token_idx: TokenIdx(
                                                                        32,
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
                                                                        33,
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
                                                                        37,
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
                                                                    21,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T1`,
                                                                            token_idx: TokenIdx(
                                                                                20,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    23,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `T2`,
                                                                            token_idx: TokenIdx(
                                                                                22,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    25,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E1`,
                                                                            token_idx: TokenIdx(
                                                                                24,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    27,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `E2`,
                                                                            token_idx: TokenIdx(
                                                                                26,
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
                                                                ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                                    51,
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
                                                    52,
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
                                                    56,
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
                                                    57,
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
                                                            50,
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
                                                            48,
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
                                                        49,
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
                                                                            ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                                                        34,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T2`,
                                                                                token_idx: TokenIdx(
                                                                                    22,
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
                                                                        35,
                                                                    ),
                                                                    current_symbol_idx: 3,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E2`,
                                                                                token_idx: TokenIdx(
                                                                                    26,
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
                                                                        38,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T1`,
                                                                                token_idx: TokenIdx(
                                                                                    20,
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
                                                                        39,
                                                                    ),
                                                                    current_symbol_idx: 2,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E1`,
                                                                                token_idx: TokenIdx(
                                                                                    24,
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
                                                                                28,
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
                                                                            29,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `ops`,
                                                                            token_idx: TokenIdx(
                                                                                30,
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
                                                                            31,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `Unveil`,
                                                                            token_idx: TokenIdx(
                                                                                32,
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
                                                                                33,
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
                                                                                37,
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
                                                                            21,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T1`,
                                                                                    token_idx: TokenIdx(
                                                                                        20,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            23,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T2`,
                                                                                    token_idx: TokenIdx(
                                                                                        22,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            25,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E1`,
                                                                                    token_idx: TokenIdx(
                                                                                        24,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            27,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E2`,
                                                                                    token_idx: TokenIdx(
                                                                                        26,
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
                                                                        ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                                            51,
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
                                                            52,
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
                                                            56,
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
                                                            57,
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
                                                                    50,
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
                                                                    48,
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
                                                                49,
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
                                                            ty_path: TypePath(`core::result::Result`, `Enum`),
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
                                                        59,
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