Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Extern(
                    ExternTypeDefn {
                        path: TypePath(`core::mem::Ref`, `Extern`),
                        decl: ExternTypeDecl {
                            path: TypePath(`core::mem::Ref`, `Extern`),
                            implicit_parameters: [
                                ImplicitParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Covariant(
                                            CovariantToken {
                                                token_idx: TokenIdx(
                                                    4,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 0,
                                    variant: ImplicitParameterDeclPatternVariant::Lifetime {
                                        label_token: LifetimeLabelToken {
                                            label: `'a`,
                                            token_idx: TokenIdx(
                                                5,
                                            ),
                                        },
                                    },
                                },
                                ImplicitParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Covariant(
                                            CovariantToken {
                                                token_idx: TokenIdx(
                                                    7,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 1,
                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `E`,
                                            token_idx: TokenIdx(
                                                8,
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
                                                        path: TypePath(`core::mem::Ref`, `Extern`),
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
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Lifetime {
                                                            label_token: LifetimeLabelToken {
                                                                label: `'a`,
                                                                token_idx: TokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    8,
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
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Extern(
                    ExternTypeDefn {
                        path: TypePath(`core::mem::RefMut`, `Extern`),
                        decl: ExternTypeDecl {
                            path: TypePath(`core::mem::RefMut`, `Extern`),
                            implicit_parameters: [
                                ImplicitParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Covariant(
                                            CovariantToken {
                                                token_idx: TokenIdx(
                                                    15,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 0,
                                    variant: ImplicitParameterDeclPatternVariant::Lifetime {
                                        label_token: LifetimeLabelToken {
                                            label: `'a`,
                                            token_idx: TokenIdx(
                                                16,
                                            ),
                                        },
                                    },
                                },
                                ImplicitParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Invariant(
                                            InvariantToken {
                                                token_idx: TokenIdx(
                                                    18,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 1,
                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `E`,
                                            token_idx: TokenIdx(
                                                19,
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
                                                        path: TypePath(`core::mem::RefMut`, `Extern`),
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
                                                        17,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Lifetime {
                                                            label_token: LifetimeLabelToken {
                                                                label: `'a`,
                                                                token_idx: TokenIdx(
                                                                    16,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        20,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    19,
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
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Extern(
                    ExternTypeDefn {
                        path: TypePath(`core::mem::Leash`, `Extern`),
                        decl: ExternTypeDecl {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            implicit_parameters: [
                                ImplicitParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Covariant(
                                            CovariantToken {
                                                token_idx: TokenIdx(
                                                    26,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 0,
                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `E`,
                                            token_idx: TokenIdx(
                                                27,
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
                                                        path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                        28,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    27,
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
                        module_path: `core::mem`,
                        trai_path: TraitPath(`core::marker::Copy`),
                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                        disambiguator: 0,
                    },
                    implicit_parameters: [
                        ImplicitParameterDecl {
                            annotated_variance_token: None,
                            symbol: 0,
                            variant: ImplicitParameterDeclPatternVariant::Type {
                                ident_token: IdentToken {
                                    ident: `E`,
                                    token_idx: TokenIdx(
                                        32,
                                    ),
                                },
                                traits: None,
                            },
                        },
                    ],
                    trai_expr: TraitExpr {
                        expr: 0,
                    },
                    ty_expr: TypeExpr {
                        expr: 3,
                    },
                    expr_region: ExprRegion {
                        data: ExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                EntityNodePath::ImplBlock(
                                    ImplBlockNodePath::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockNodePath {
                                            path: TraitForTypeImplBlockPath {
                                                module_path: `core::mem`,
                                                trai_path: TraitPath(`core::marker::Copy`),
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::marker::Copy`),
                                                ),
                                            ),
                                        ),
                                    },
                                    Expr::PrincipalEntityPath {
                                        entity_path_expr: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::mem::Leash`, `Extern`),
                                                ),
                                            ),
                                        ),
                                    },
                                    Expr::CurrentSymbol {
                                        ident: `E`,
                                        token_idx: TokenIdx(
                                            37,
                                        ),
                                        current_symbol_idx: 0,
                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `E`,
                                                    token_idx: TokenIdx(
                                                        32,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    Expr::ExplicitApplication {
                                        function: 1,
                                        argument: 2,
                                    },
                                ],
                            },
                            principal_entity_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `Copy`,
                                                token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Trait(
                                                TraitPath(`core::marker::Copy`),
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `Leash`,
                                                token_idx: TokenIdx(
                                                    36,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`core::mem::Leash`, `Extern`),
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
                                                33,
                                            ),
                                            access_end: None,
                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            32,
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
                                ],
                            },
                            roots: [
                                ExprRoot {
                                    kind: Trait,
                                    expr_idx: 0,
                                },
                                ExprRoot {
                                    kind: SelfType,
                                    expr_idx: 3,
                                },
                            ],
                        },
                    },
                },
            ),
        ),
    ],
)