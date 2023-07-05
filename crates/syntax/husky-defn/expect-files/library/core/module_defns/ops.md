Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::Add`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::Add`),
                        ast_idx: 30,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            11,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Add`),
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
                                                    12,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::AddAssign`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::AddAssign`),
                        ast_idx: 32,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            40,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::AddAssign`),
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
                                                    41,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                40,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::BitAnd`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::BitAnd`),
                        ast_idx: 34,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            69,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitAnd`),
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
                                                    70,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                69,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::BitAndAssign`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::BitAndAssign`),
                        ast_idx: 36,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            98,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitAndAssign`),
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
                                                    99,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                98,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::BitOr`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::BitOr`),
                        ast_idx: 38,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            123,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitOr`),
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
                                                    124,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                123,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::BitOrAssign`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::BitOrAssign`),
                        ast_idx: 40,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            152,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitOrAssign`),
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
                                                    153,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                152,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::BitXor`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::BitXor`),
                        ast_idx: 42,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            177,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitXor`),
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
                                                    178,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                177,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::BitXorAssign`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::BitXorAssign`),
                        ast_idx: 44,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            206,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitXorAssign`),
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
                                                    207,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                206,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::Div`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::Div`),
                        ast_idx: 46,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            231,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Div`),
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
                                                    232,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                231,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::DivAssign`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::DivAssign`),
                        ast_idx: 48,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            260,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::DivAssign`),
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
                                                    261,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                260,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::Mul`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::Mul`),
                        ast_idx: 50,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            285,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Mul`),
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
                                                    286,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                285,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::MulAssign`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::MulAssign`),
                        ast_idx: 52,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            314,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::MulAssign`),
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
                                                    315,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                314,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::Neg`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::Neg`),
                        ast_idx: 54,
                        implicit_parameters: [],
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Neg`),
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
                },
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::Not`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::Not`),
                        ast_idx: 56,
                        implicit_parameters: [],
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Not`),
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
                },
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::Sub`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::Sub`),
                        ast_idx: 58,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `Rhs`,
                                        token_idx: TokenIdx(
                                            387,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Sub`),
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
                                                    388,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                387,
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
        Defn::ModuleItem(
            ModuleItemDefn::Trait(
                TraitDefn {
                    path: TraitPath(`core::ops::Unveil`),
                    decl: TraitDecl {
                        path: TraitPath(`core::ops::Unveil`),
                        ast_idx: 59,
                        implicit_parameters: [
                            ImplicitParameterDecl {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: ImplicitParameterDeclPatternVariant::Type {
                                    ident_token: IdentToken {
                                        ident: `T`,
                                        token_idx: TokenIdx(
                                            409,
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
                                        ModuleItemNodePath::Trait(
                                            TraitNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Unveil`),
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
                                                    410,
                                                ),
                                                access_end: None,
                                                variant: CurrentSymbolVariant::ImplicitParameter {
                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                        ident_token: IdentToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                409,
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
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Enum(
                    EnumTypeDefn {
                        path: TypePath(`core::ops::ControlFlow`, `Enum`),
                        decl: EnumTypeDecl {
                            path: TypePath(`core::ops::ControlFlow`, `Enum`),
                            implicit_parameters: [
                                ImplicitParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `R`,
                                            token_idx: TokenIdx(
                                                438,
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
                                            ident: `C`,
                                            token_idx: TokenIdx(
                                                440,
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
                                                        path: TypePath(`core::ops::ControlFlow`, `Enum`),
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
                                                        439,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `R`,
                                                                token_idx: TokenIdx(
                                                                    438,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        441,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `C`,
                                                                token_idx: TokenIdx(
                                                                    440,
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
    ],
)