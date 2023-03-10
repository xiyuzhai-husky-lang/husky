Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::Add`),
                            ast_idx: 26,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::Add`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                10,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                11,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    12,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::AddAssign`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::AddAssign`),
                            ast_idx: 28,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::AddAssign`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                39,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                40,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    41,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::BitAnd`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::BitAnd`),
                            ast_idx: 30,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::BitAnd`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                68,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                69,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    70,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::BitAndAssign`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::BitAndAssign`),
                            ast_idx: 32,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::BitAndAssign`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                97,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                98,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    99,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::BitOr`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::BitOr`),
                            ast_idx: 34,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::BitOr`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                122,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                123,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    124,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::BitOrAssign`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::BitOrAssign`),
                            ast_idx: 36,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::BitOrAssign`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                151,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                152,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    153,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::BitXor`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::BitXor`),
                            ast_idx: 38,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::BitXor`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                176,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                177,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    178,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::BitXorAssign`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::BitXorAssign`),
                            ast_idx: 40,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::BitXorAssign`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                205,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                206,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    207,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Div`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::Div`),
                            ast_idx: 42,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::Div`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                230,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                231,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    232,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::DivAssign`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::DivAssign`),
                            ast_idx: 44,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::DivAssign`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                259,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                260,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    261,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Mul`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::Mul`),
                            ast_idx: 46,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::Mul`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                284,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                285,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    286,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::MulAssign`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::MulAssign`),
                            ast_idx: 48,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::MulAssign`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                313,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                314,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    315,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Neg`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::Neg`),
                            ast_idx: 50,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::Neg`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Not`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::Not`),
                            ast_idx: 52,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::Not`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                        },
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Sub`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Trait(
                        TraitDecl {
                            path: TraitPath(`core::ops::Sub`),
                            ast_idx: 54,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclRegionPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::Sub`),
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
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
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                386,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                pattern: ImplicitParameterDeclPattern {
                                                    annotated_variance_token: None,
                                                    symbol: 0,
                                                    variant: Type0 {
                                                        ident_token: IdentToken {
                                                            ident: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 60,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                387,
                                                            ),
                                                        },
                                                    },
                                                },
                                                traits: None,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: Ok(
                                            RightAngleBracketToken(
                                                TokenIdx(
                                                    388,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)