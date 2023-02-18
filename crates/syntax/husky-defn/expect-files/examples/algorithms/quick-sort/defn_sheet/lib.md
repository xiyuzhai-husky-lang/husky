Ok(
    DefnSheet {
        defns: [
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`quick_sort::quick_sort`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`quick_sort::quick_sort`, `Function`),
                            ast_idx: 30,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`quick_sort::quick_sort`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TraitPath(`core::cmp::Ord`),
                                                ),
                                            },
                                            Expr::BoxColon {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    12,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    13,
                                                ),
                                                rbox_token: RightBoxBracketToken {
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `T`,
                                                token_idx: TokenIdx(
                                                    15,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: ImplicitParameterKind::Type {
                                                        ident_token: IdentifierToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            Expr::Application {
                                                function: 1,
                                                argument: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    6,
                                                ),
                                                ident: `Ord`,
                                                entity_path: TraitPath(`core::cmp::Ord`),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `arr`,
                                                        token_idx: TokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 80,
                                                            },
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                PatternSymbol::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    ident: `T`,
                                                    access_start: TokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                            ident_token: IdentifierToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `arr`,
                                                    access_start: TokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            ImplicitTypeParameter,
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 3,
                                            },
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            3,
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
                                                                    value: 79,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                },
                            ),
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        8,
                                    ),
                                },
                                parameters: [
                                    RegularParameterDeclPattern {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                11,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        16,
                                    ),
                                },
                            },
                            curry_token: Err(
                                MissingCurry(
                                    TokenIdx(
                                        17,
                                    ),
                                ),
                            ),
                            return_ty: Err(
                                MissingOutputType(
                                    TokenIdx(
                                        17,
                                    ),
                                ),
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        17,
                                    ),
                                },
                            ),
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclExprPath::Entity(
                                                    FormPath(`quick_sort::quick_sort`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    },
                                                    Expr::BoxColon {
                                                        caller: None,
                                                        lbox_token_idx: TokenIdx(
                                                            12,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            13,
                                                        ),
                                                        rbox_token: RightBoxBracketToken {
                                                            token_idx: TokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            15,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: ImplicitParameterKind::Type {
                                                                ident_token: IdentifierToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::Application {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                        ident: `Ord`,
                                                        entity_path: TraitPath(`core::cmp::Ord`),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `arr`,
                                                                token_idx: TokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 80,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
                                                    ],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            ident: `T`,
                                                            access_start: TokenIdx(
                                                                5,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                                    ident_token: IdentifierToken {
                                                                        ident: `T`,
                                                                        token_idx: TokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `arr`,
                                                            access_start: TokenIdx(
                                                                11,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    ImplicitTypeParameter,
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 3,
                                                    },
                                                ],
                                            },
                                            roots: [],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`quick_sort::quick_sort`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                21,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                22,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `len`,
                                                token_idx: TokenIdx(
                                                    23,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                24,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                25,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `len`,
                                            token_idx: TokenIdx(
                                                33,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                35,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                34,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                32,
                                            ),
                                            item: 5,
                                            rpar_token_idx: TokenIdx(
                                                36,
                                            ),
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 60,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                28,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                30,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 6,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                37,
                                            ),
                                            ropd: 7,
                                        },
                                        Expr::RitchieCall {
                                            function: 2,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                27,
                                            ),
                                            arguments: ArenaIdxRange(
                                                8..11,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                39,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..2,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                26,
                                            ),
                                            ident: `quick_sort_aux`,
                                            entity_path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    18,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        20,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                1,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 11,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `len`,
                                                    token_idx: TokenIdx(
                                                        19,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 81,
                                                        },
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                ident: `T`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::ImplicitParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `arr`,
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `len`,
                                                access_start: TokenIdx(
                                                    20,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            40,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 12,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            12,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                            ast_idx: 31,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TraitPath(`core::cmp::Ord`),
                                                ),
                                            },
                                            Expr::BoxColon {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    51,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    52,
                                                ),
                                                rbox_token: RightBoxBracketToken {
                                                    token_idx: TokenIdx(
                                                        53,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `T`,
                                                token_idx: TokenIdx(
                                                    54,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: ImplicitParameterKind::Type {
                                                        ident_token: IdentifierToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                43,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            Expr::Application {
                                                function: 1,
                                                argument: 2,
                                            },
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 60,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            62,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 60,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    45,
                                                ),
                                                ident: `Ord`,
                                                entity_path: TraitPath(`core::cmp::Ord`),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `arr`,
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `low`,
                                                        token_idx: TokenIdx(
                                                            56,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `high`,
                                                        token_idx: TokenIdx(
                                                            60,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                            Parameter,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 80,
                                                            },
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 83,
                                                            },
                                                        ),
                                                    ),
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 84,
                                                            },
                                                        ),
                                                    ),
                                                    2,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                PatternSymbol::Atom(
                                                    0,
                                                ),
                                                PatternSymbol::Atom(
                                                    1,
                                                ),
                                                PatternSymbol::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    ident: `T`,
                                                    access_start: TokenIdx(
                                                        44,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                            ident_token: IdentifierToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    43,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `arr`,
                                                    access_start: TokenIdx(
                                                        50,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `low`,
                                                    access_start: TokenIdx(
                                                        57,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `high`,
                                                    access_start: TokenIdx(
                                                        61,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            ImplicitTypeParameter,
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 3,
                                            },
                                            RegularParameter {
                                                pattern: 1,
                                                ty: 4,
                                            },
                                            RegularParameter {
                                                pattern: 2,
                                                ty: 5,
                                            },
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            42,
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
                                                                    value: 79,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            43,
                                                        ),
                                                    },
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            44,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            46,
                                        ),
                                    },
                                },
                            ),
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        47,
                                    ),
                                },
                                parameters: [
                                    RegularParameterDeclPattern {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                50,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                57,
                                            ),
                                        },
                                        ty: 4,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 2,
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                61,
                                            ),
                                        },
                                        ty: 5,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            59,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        63,
                                    ),
                                },
                            },
                            curry_token: Err(
                                MissingCurry(
                                    TokenIdx(
                                        64,
                                    ),
                                ),
                            ),
                            return_ty: Err(
                                MissingOutputType(
                                    TokenIdx(
                                        64,
                                    ),
                                ),
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        64,
                                    ),
                                },
                            ),
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclExprPath::Entity(
                                                    FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    },
                                                    Expr::BoxColon {
                                                        caller: None,
                                                        lbox_token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                        rbox_token: RightBoxBracketToken {
                                                            token_idx: TokenIdx(
                                                                53,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            54,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: ImplicitParameterKind::Type {
                                                                ident_token: IdentifierToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        43,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::Application {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                    Expr::Err(
                                                        Original(
                                                            UnrecognizedIdentifier {
                                                                token_idx: TokenIdx(
                                                                    58,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Err(
                                                        Original(
                                                            UnrecognizedIdentifier {
                                                                token_idx: TokenIdx(
                                                                    62,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                        ident: `Ord`,
                                                        entity_path: TraitPath(`core::cmp::Ord`),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `arr`,
                                                                token_idx: TokenIdx(
                                                                    49,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `low`,
                                                                token_idx: TokenIdx(
                                                                    56,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `high`,
                                                                token_idx: TokenIdx(
                                                                    60,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 80,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 83,
                                                                    },
                                                                ),
                                                            ),
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 84,
                                                                    },
                                                                ),
                                                            ),
                                                            2,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
                                                        PatternSymbol::Atom(
                                                            1,
                                                        ),
                                                        PatternSymbol::Atom(
                                                            2,
                                                        ),
                                                    ],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            ident: `T`,
                                                            access_start: TokenIdx(
                                                                44,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                                    ident_token: IdentifierToken {
                                                                        ident: `T`,
                                                                        token_idx: TokenIdx(
                                                                            43,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `arr`,
                                                            access_start: TokenIdx(
                                                                50,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `low`,
                                                            access_start: TokenIdx(
                                                                57,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `high`,
                                                            access_start: TokenIdx(
                                                                61,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    ImplicitTypeParameter,
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 3,
                                                    },
                                                    RegularParameter {
                                                        pattern: 1,
                                                        ty: 4,
                                                    },
                                                    RegularParameter {
                                                        pattern: 2,
                                                        ty: 5,
                                                    },
                                                ],
                                            },
                                            roots: [],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                66,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                68,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 0,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                67,
                                            ),
                                            ropd: 1,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`quick_sort::partition`, `Function`),
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                75,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                77,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                79,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::RitchieCall {
                                            function: 3,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                74,
                                            ),
                                            arguments: ArenaIdxRange(
                                                4..7,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                80,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `p`,
                                            token_idx: TokenIdx(
                                                87,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                89,
                                            ),
                                        ),
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                83,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                85,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 9,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                88,
                                            ),
                                            ropd: 10,
                                        },
                                        Expr::RitchieCall {
                                            function: 8,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                82,
                                            ),
                                            arguments: ArenaIdxRange(
                                                11..14,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                90,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `p`,
                                            token_idx: TokenIdx(
                                                95,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                97,
                                            ),
                                        ),
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                93,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 16,
                                            opr: PureClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                96,
                                            ),
                                            ropd: 17,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                99,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::RitchieCall {
                                            function: 15,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                92,
                                            ),
                                            arguments: ArenaIdxRange(
                                                18..21,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                100,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                3..4,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                73,
                                            ),
                                            ident: `partition`,
                                            entity_path: FormPath(`quick_sort::partition`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                81,
                                            ),
                                            ident: `quick_sort_aux`,
                                            entity_path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                91,
                                            ),
                                            ident: `quick_sort_aux`,
                                            entity_path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    70,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                7,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 14,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 21,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        65,
                                                    ),
                                                },
                                                condition: Ok(
                                                    2,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            69,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..3,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `p`,
                                                    token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 85,
                                                        },
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                ident: `T`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::ImplicitParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `arr`,
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `low`,
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `high`,
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `p`,
                                                access_start: TokenIdx(
                                                    72,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            101,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 22,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            22,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`quick_sort::partition`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`quick_sort::partition`, `Function`),
                            ast_idx: 32,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`quick_sort::partition`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TraitPath(`core::cmp::Ord`),
                                                ),
                                            },
                                            Expr::BoxColon {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    112,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    113,
                                                ),
                                                rbox_token: RightBoxBracketToken {
                                                    token_idx: TokenIdx(
                                                        114,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `T`,
                                                token_idx: TokenIdx(
                                                    115,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: ImplicitParameterKind::Type {
                                                        ident_token: IdentifierToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                104,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            Expr::Application {
                                                function: 1,
                                                argument: 2,
                                            },
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            119,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 60,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            123,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 60,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            126,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 60,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    106,
                                                ),
                                                ident: `Ord`,
                                                entity_path: TraitPath(`core::cmp::Ord`),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `arr`,
                                                        token_idx: TokenIdx(
                                                            110,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `low`,
                                                        token_idx: TokenIdx(
                                                            117,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `high`,
                                                        token_idx: TokenIdx(
                                                            121,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                            Parameter,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 80,
                                                            },
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 83,
                                                            },
                                                        ),
                                                    ),
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 84,
                                                            },
                                                        ),
                                                    ),
                                                    2,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                PatternSymbol::Atom(
                                                    0,
                                                ),
                                                PatternSymbol::Atom(
                                                    1,
                                                ),
                                                PatternSymbol::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    ident: `T`,
                                                    access_start: TokenIdx(
                                                        105,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                            ident_token: IdentifierToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    104,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `arr`,
                                                    access_start: TokenIdx(
                                                        111,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `low`,
                                                    access_start: TokenIdx(
                                                        118,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `high`,
                                                    access_start: TokenIdx(
                                                        122,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            ImplicitTypeParameter,
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 3,
                                            },
                                            RegularParameter {
                                                pattern: 1,
                                                ty: 4,
                                            },
                                            RegularParameter {
                                                pattern: 2,
                                                ty: 5,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr: 6,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: Some(
                                ImplicitParameterDeclList {
                                    langle: LeftAngleBracketOrLessThanToken {
                                        token_idx: TokenIdx(
                                            103,
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
                                                                    value: 79,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            104,
                                                        ),
                                                    },
                                                },
                                            },
                                            traits: Some(
                                                (
                                                    ColonToken {
                                                        token_idx: TokenIdx(
                                                            105,
                                                        ),
                                                    },
                                                    Some(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                    commas: [],
                                    rangle: RightAngleBracketToken {
                                        token_idx: TokenIdx(
                                            107,
                                        ),
                                    },
                                },
                            ),
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        108,
                                    ),
                                },
                                parameters: [
                                    RegularParameterDeclPattern {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                111,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                118,
                                            ),
                                        },
                                        ty: 4,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 2,
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                122,
                                            ),
                                        },
                                        ty: 5,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            116,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            120,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        124,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        125,
                                    ),
                                },
                            ),
                            return_ty: Ok(
                                OutputTypeExpr {
                                    expr: 6,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        127,
                                    ),
                                },
                            ),
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclExprPath::Entity(
                                                    FormPath(`quick_sort::partition`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    },
                                                    Expr::BoxColon {
                                                        caller: None,
                                                        lbox_token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            113,
                                                        ),
                                                        rbox_token: RightBoxBracketToken {
                                                            token_idx: TokenIdx(
                                                                114,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            115,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: ImplicitParameterKind::Type {
                                                                ident_token: IdentifierToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        104,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::Application {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                    Expr::Err(
                                                        Original(
                                                            UnrecognizedIdentifier {
                                                                token_idx: TokenIdx(
                                                                    119,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Err(
                                                        Original(
                                                            UnrecognizedIdentifier {
                                                                token_idx: TokenIdx(
                                                                    123,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Err(
                                                        Original(
                                                            UnrecognizedIdentifier {
                                                                token_idx: TokenIdx(
                                                                    126,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            106,
                                                        ),
                                                        ident: `Ord`,
                                                        entity_path: TraitPath(`core::cmp::Ord`),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `arr`,
                                                                token_idx: TokenIdx(
                                                                    110,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `low`,
                                                                token_idx: TokenIdx(
                                                                    117,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `high`,
                                                                token_idx: TokenIdx(
                                                                    121,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 80,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 83,
                                                                    },
                                                                ),
                                                            ),
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 84,
                                                                    },
                                                                ),
                                                            ),
                                                            2,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
                                                        PatternSymbol::Atom(
                                                            1,
                                                        ),
                                                        PatternSymbol::Atom(
                                                            2,
                                                        ),
                                                    ],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            ident: `T`,
                                                            access_start: TokenIdx(
                                                                105,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: ImplicitParameterVariant::Type {
                                                                    ident_token: IdentifierToken {
                                                                        ident: `T`,
                                                                        token_idx: TokenIdx(
                                                                            104,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `arr`,
                                                            access_start: TokenIdx(
                                                                111,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `low`,
                                                            access_start: TokenIdx(
                                                                118,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `high`,
                                                            access_start: TokenIdx(
                                                                122,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    ImplicitTypeParameter,
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 3,
                                                    },
                                                    RegularParameter {
                                                        pattern: 1,
                                                        ty: 4,
                                                    },
                                                    RegularParameter {
                                                        pattern: 2,
                                                        ty: 5,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 6,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`quick_sort::partition`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                131,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        133,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 66,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 0,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                132,
                                            ),
                                            ropd: 1,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                138,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                140,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                139,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                145,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                147,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                151,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 8,
                                            opr: Assign(
                                                Some(
                                                    Add,
                                                ),
                                            ),
                                            opr_token_idx: TokenIdx(
                                                150,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                153,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                155,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 66,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 12,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                156,
                                            ),
                                            ropd: 13,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                160,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                162,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                11,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                154,
                                            ),
                                            items: ArenaIdxRange(
                                                14..15,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                158,
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                15,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                161,
                                            ),
                                            items: ArenaIdxRange(
                                                16..17,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                163,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 17,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                159,
                                            ),
                                            ropd: 18,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                165,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                167,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 20,
                                            opr: Assign(
                                                Some(
                                                    Add,
                                                ),
                                            ),
                                            opr_token_idx: TokenIdx(
                                                166,
                                            ),
                                            ropd: 21,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                168,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                170,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 23,
                                            opr: Assign(
                                                Some(
                                                    Sub,
                                                ),
                                            ),
                                            opr_token_idx: TokenIdx(
                                                169,
                                            ),
                                            ropd: 24,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                172,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                174,
                                            ),
                                        ),
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                176,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                178,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        180,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 66,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 29,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                179,
                                            ),
                                            ropd: 30,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                183,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                185,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                28,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                177,
                                            ),
                                            items: ArenaIdxRange(
                                                31..32,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                181,
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                32,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                184,
                                            ),
                                            items: ArenaIdxRange(
                                                33..34,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                186,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 26,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                173,
                                            ),
                                            ropd: 27,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 34,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ropd: 35,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 36,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                175,
                                            ),
                                            ropd: 37,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                188,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                190,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 39,
                                            opr: Assign(
                                                Some(
                                                    Sub,
                                                ),
                                            ),
                                            opr_token_idx: TokenIdx(
                                                189,
                                            ),
                                            ropd: 40,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                192,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                194,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 42,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                193,
                                            ),
                                            ropd: 43,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                199,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                203,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 66,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                207,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 66,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 46,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                204,
                                            ),
                                            ropd: 47,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 48,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                208,
                                            ),
                                            ropd: 49,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 45,
                                            dot_token_idx: TokenIdx(
                                                200,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `swap`,
                                                token_idx: TokenIdx(
                                                    201,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                202,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                50..52,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                210,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                211,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                215,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 66,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                219,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 66,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 54,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                216,
                                            ),
                                            ropd: 55,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 56,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                220,
                                            ),
                                            ropd: 57,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 53,
                                            dot_token_idx: TokenIdx(
                                                212,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `swap`,
                                                token_idx: TokenIdx(
                                                    213,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                214,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                58..60,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                222,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                223,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                9..15,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 22,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 41,
                                        },
                                        Stmt::Break {
                                            break_token: BreakToken {
                                                token_idx: TokenIdx(
                                                    196,
                                                ),
                                            },
                                        },
                                        Stmt::Eval {
                                            expr_idx: 52,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 10,
                                        },
                                        Stmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    152,
                                                ),
                                            },
                                            condition: Ok(
                                                19,
                                            ),
                                            eol_colon: Ok(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        164,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 25,
                                        },
                                        Stmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    171,
                                                ),
                                            },
                                            condition: Ok(
                                                38,
                                            ),
                                            eol_colon: Ok(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        187,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                },
                                                condition: Ok(
                                                    44,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            195,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                ElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            197,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                198,
                                                            ),
                                                        },
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            3..4,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    128,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                2,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 1,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        137,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                5,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    141,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 2,
                                                    variables: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        144,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                6,
                                            ),
                                        },
                                        Stmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    146,
                                                ),
                                            },
                                            condition: Ok(
                                                7,
                                            ),
                                            eol_colon: Ok(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    4..9,
                                                ),
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 60,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 61,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `pivot`,
                                                    token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `store_index`,
                                                    token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `last_index`,
                                                    token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                        Let,
                                        Let,
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 87,
                                                        },
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 88,
                                                        },
                                                    ),
                                                ),
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 89,
                                                        },
                                                    ),
                                                ),
                                                2,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                            PatternSymbol::Atom(
                                                1,
                                            ),
                                            PatternSymbol::Atom(
                                                2,
                                            ),
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                ident: `T`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::ImplicitParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `arr`,
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `low`,
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `high`,
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `pivot`,
                                                access_start: TokenIdx(
                                                    130,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `store_index`,
                                                access_start: TokenIdx(
                                                    137,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `last_index`,
                                                access_start: TokenIdx(
                                                    144,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 62,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            62,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                            ast_idx: 34,
                            curry_token: Err(
                                MissingCurry(
                                    TokenIdx(
                                        228,
                                    ),
                                ),
                            ),
                            return_ty: Err(
                                MissingOutputType(
                                    TokenIdx(
                                        228,
                                    ),
                                ),
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        228,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
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
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [],
                                },
                            },
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Literal(
                                            TokenIdx(
                                                241,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                234,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                236,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                238,
                                            ),
                                        ),
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                240,
                                            ),
                                            opd: 0,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                243,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                245,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                247,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                249,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                251,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                253,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                233,
                                            ),
                                            items: ArenaIdxRange(
                                                1..11,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                254,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`quick_sort::quick_sort`, `Function`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `v`,
                                            token_idx: TokenIdx(
                                                257,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::ApplicationOrRitchieCall {
                                            function: 12,
                                            lpar_token_idx: TokenIdx(
                                                256,
                                            ),
                                            argument: 13,
                                            rpar_token_idx: TokenIdx(
                                                258,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                264,
                                            ),
                                        ),
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                263,
                                            ),
                                            opd: 15,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                266,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                268,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                270,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                272,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                274,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                276,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                278,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                280,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                282,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `v`,
                                            token_idx: TokenIdx(
                                                260,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                262,
                                            ),
                                            items: ArenaIdxRange(
                                                16..26,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                283,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 26,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                261,
                                            ),
                                            ropd: 27,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                255,
                                            ),
                                            ident: `quick_sort`,
                                            entity_path: FormPath(`quick_sort::quick_sort`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    229,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        232,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                11,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 14,
                                        },
                                        Stmt::Assert {
                                            assert_token: AssertToken {
                                                token_idx: TokenIdx(
                                                    259,
                                                ),
                                            },
                                            condition: Ok(
                                                28,
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `v`,
                                                    token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 93,
                                                        },
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `v`,
                                                access_start: TokenIdx(
                                                    232,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            284,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 29,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            29,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                            ast_idx: 36,
                            curry_token: Err(
                                MissingCurry(
                                    TokenIdx(
                                        288,
                                    ),
                                ),
                            ),
                            return_ty: Err(
                                MissingOutputType(
                                    TokenIdx(
                                        288,
                                    ),
                                ),
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        288,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
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
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [],
                                },
                            },
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Literal(
                                            TokenIdx(
                                                294,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                296,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                298,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                300,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                302,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                304,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                293,
                                            ),
                                            items: ArenaIdxRange(
                                                0..6,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                305,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`quick_sort::quick_sort`, `Function`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `strs`,
                                            token_idx: TokenIdx(
                                                308,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::ApplicationOrRitchieCall {
                                            function: 7,
                                            lpar_token_idx: TokenIdx(
                                                307,
                                            ),
                                            argument: 8,
                                            rpar_token_idx: TokenIdx(
                                                309,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                314,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                316,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                318,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                320,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                322,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                324,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `strs`,
                                            token_idx: TokenIdx(
                                                311,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                313,
                                            ),
                                            items: ArenaIdxRange(
                                                10..16,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                325,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 16,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                312,
                                            ),
                                            ropd: 17,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                306,
                                            ),
                                            ident: `quick_sort`,
                                            entity_path: FormPath(`quick_sort::quick_sort`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    289,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        292,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                6,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 9,
                                        },
                                        Stmt::Assert {
                                            assert_token: AssertToken {
                                                token_idx: TokenIdx(
                                                    310,
                                                ),
                                            },
                                            condition: Ok(
                                                18,
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `strs`,
                                                    token_idx: TokenIdx(
                                                        291,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 95,
                                                        },
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `strs`,
                                                access_start: TokenIdx(
                                                    292,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            326,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 19,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            19,
                        ),
                    },
                ),
            ),
        ],
    },
)