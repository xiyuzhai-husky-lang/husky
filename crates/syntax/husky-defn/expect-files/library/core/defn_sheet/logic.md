Ok(
    DefnSheet {
        defns: [
            Type(
                Foreign(
                    AlienTypeDefn {
                        path: TypePath(`core::logic::Prop`, `Alien`),
                        decl: AlienTypeDecl {
                            path: TypePath(`core::logic::Prop`, `Alien`),
                            ast_idx: 0,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            TypePath(`core::logic::Prop`, `Alien`),
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
                                        ty_constraints: [],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: None,
                        },
                    },
                ),
            ),
            Type(
                Structure(
                    StructureTypeDefn {
                        path: TypePath(`core::logic::LogicAnd`, `Structure`),
                        decl: StructureTypeDecl {
                            path: TypePath(`core::logic::LogicAnd`, `Structure`),
                            ast_idx: 1,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            TypePath(`core::logic::LogicAnd`, `Structure`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::logic::Prop`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::logic::Prop`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    10,
                                                ),
                                                ident: `Prop`,
                                                entity_path: TypePath(`core::logic::Prop`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    14,
                                                ),
                                                ident: `Prop`,
                                                entity_path: TypePath(`core::logic::Prop`, `Alien`),
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
                                                    ident: `P`,
                                                    access_start: TokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                            ident_token: IdentifierToken {
                                                                ident: `P`,
                                                                token_idx: TokenIdx(
                                                                    8,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `Q`,
                                                    access_start: TokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                            ident_token: IdentifierToken {
                                                                ident: `Q`,
                                                                token_idx: TokenIdx(
                                                                    12,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            ImplicitTypeParameter,
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                    implicit_parameters: [
                                        ImplicitParameterDecl {
                                            pattern: ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: Type0 {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                        ImplicitParameterDecl {
                                            pattern: ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: Type0 {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 18,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                    Some(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                11,
                                            ),
                                        },
                                    ],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                },
                            ),
                        },
                    },
                ),
            ),
            Type(
                Inductive(
                    InductiveTypeDefn {
                        path: TypePath(`core::logic::LogicOr`, `Inductive`),
                        decl: InductiveTypeDecl {
                            path: TypePath(`core::logic::LogicOr`, `Inductive`),
                            ast_idx: 2,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            TypePath(`core::logic::LogicOr`, `Inductive`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::logic::Prop`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::logic::Prop`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    32,
                                                ),
                                                ident: `Prop`,
                                                entity_path: TypePath(`core::logic::Prop`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    36,
                                                ),
                                                ident: `Prop`,
                                                entity_path: TypePath(`core::logic::Prop`, `Alien`),
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
                                                    ident: `P`,
                                                    access_start: TokenIdx(
                                                        31,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                            ident_token: IdentifierToken {
                                                                ident: `P`,
                                                                token_idx: TokenIdx(
                                                                    30,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `Q`,
                                                    access_start: TokenIdx(
                                                        35,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                            ident_token: IdentifierToken {
                                                                ident: `Q`,
                                                                token_idx: TokenIdx(
                                                                    34,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            ImplicitTypeParameter,
                                            ImplicitTypeParameter,
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            29,
                                        ),
                                    },
                                    implicit_parameters: [
                                        ImplicitParameterDecl {
                                            pattern: ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: Type0 {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 17,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            30,
                                                        ),
                                                    },
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                        ImplicitParameterDecl {
                                            pattern: ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: Type0 {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 18,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            35,
                                                        ),
                                                    },
                                                    Some(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                33,
                                            ),
                                        },
                                    ],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            37,
                                        ),
                                    },
                                },
                            ),
                        },
                    },
                ),
            ),
        ],
    },
)