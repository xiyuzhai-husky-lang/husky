Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::quick_sort`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`quick_sort::quick_sort`, `Function`),
                                decl: FunctionDecl {
                                    path: FormPath(`quick_sort::quick_sort`, `Function`),
                                    ast_idx: 30,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::quick_sort`, `Function`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            12,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            13,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            15,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                ident_token: IdentifierToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        4,
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
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                        ident: `Ord`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
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
                                                                        value: 108,
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
                                                            access_start: TokenIdx(
                                                                5,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
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
                                                            access_start: TokenIdx(
                                                                11,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `arr`,
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
                                    implicit_parameter_decl_list: Ok(
                                        Some(
                                            ImplicitParameterDeclList {
                                                langle: LeftAngleBracketOrLessThanToken(
                                                    TokenIdx(
                                                        3,
                                                    ),
                                                ),
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
                                                                                value: 107,
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
                                                                ColonToken(
                                                                    TokenIdx(
                                                                        5,
                                                                    ),
                                                                ),
                                                                Some(
                                                                    0,
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rangle: Ok(
                                                    RightAngleBracketToken(
                                                        TokenIdx(
                                                            7,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                    parameter_decl_list: Ok(
                                        ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    8,
                                                ),
                                            ),
                                            parameters: [
                                                ExplicitParameterDeclPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            11,
                                                        ),
                                                    ),
                                                    ty: 3,
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rpar: Ok(
                                                RightParenthesisToken(
                                                    TokenIdx(
                                                        16,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Err(
                                        Original(
                                            ExpectCurry(
                                                TokenIdx(
                                                    17,
                                                ),
                                            ),
                                        ),
                                    ),
                                    return_ty: Err(
                                        Original(
                                            ExpectOutputType(
                                                TokenIdx(
                                                    17,
                                                ),
                                            ),
                                        ),
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                17,
                                            ),
                                        ),
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FormPath(`quick_sort::quick_sort`, `Function`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::cmp::Ord`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::BoxColonList {
                                                                lbox_token_idx: TokenIdx(
                                                                    12,
                                                                ),
                                                                colon_token_idx: TokenIdx(
                                                                    13,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    1..1,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    14,
                                                                ),
                                                            },
                                                            Expr::CurrentSymbol {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    15,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                        ident_token: IdentifierToken {
                                                                            ident: `T`,
                                                                            token_idx: TokenIdx(
                                                                                4,
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
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    6,
                                                                ),
                                                                ident: `Ord`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::cmp::Ord`),
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
                                                                                value: 108,
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
                                                                    access_start: TokenIdx(
                                                                        5,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
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
                                                                    access_start: TokenIdx(
                                                                        11,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `arr`,
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
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::quick_sort`, `Function`),
                                                    ),
                                                ),
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
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                20,
                                                            ),
                                                        },
                                                    ),
                                                ),
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
                                                Expr::BinaryOpn {
                                                    lopd: 1,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        20,
                                                    ),
                                                    ropd: 2,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                            ),
                                                        ),
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
                                                    lopd: 5,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                    ropd: 6,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        32,
                                                    ),
                                                    item: 7,
                                                    rpar_token_idx: TokenIdx(
                                                        36,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `arr`,
                                                    token_idx: TokenIdx(
                                                        28,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        30,
                                                    ),
                                                ),
                                                Expr::BinaryOpn {
                                                    lopd: 8,
                                                    opr: As,
                                                    opr_token_idx: TokenIdx(
                                                        37,
                                                    ),
                                                    ropd: 9,
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 4,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        27,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        10..13,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            29,
                                                        ),
                                                        TokenIdx(
                                                            31,
                                                        ),
                                                    ],
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
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                    ident: `isize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
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
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    20,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        3,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 13,
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
                                                                    value: 109,
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
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        kind: InheritedSymbolKind::ImplicitParameter(
                                                            Type {
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 107,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `arr`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
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
                                                            ident: `len`,
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
                                                expr: 14,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    14,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::quick_sort_aux`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                decl: FunctionDecl {
                                    path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                    ast_idx: 31,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            54,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                ident_token: IdentifierToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        43,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                        ident: `Ord`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                        ident: `isize`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            62,
                                                        ),
                                                        ident: `isize`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                                        value: 108,
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
                                                                        value: 111,
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
                                                                        value: 112,
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
                                                            access_start: TokenIdx(
                                                                44,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
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
                                                            access_start: TokenIdx(
                                                                50,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                57,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `low`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                61,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `high`,
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
                                    implicit_parameter_decl_list: Ok(
                                        Some(
                                            ImplicitParameterDeclList {
                                                langle: LeftAngleBracketOrLessThanToken(
                                                    TokenIdx(
                                                        42,
                                                    ),
                                                ),
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
                                                                                value: 107,
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
                                                                ColonToken(
                                                                    TokenIdx(
                                                                        44,
                                                                    ),
                                                                ),
                                                                Some(
                                                                    0,
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rangle: Ok(
                                                    RightAngleBracketToken(
                                                        TokenIdx(
                                                            46,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                    parameter_decl_list: Ok(
                                        ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    47,
                                                ),
                                            ),
                                            parameters: [
                                                ExplicitParameterDeclPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            50,
                                                        ),
                                                    ),
                                                    ty: 3,
                                                },
                                                ExplicitParameterDeclPattern {
                                                    pattern: 1,
                                                    variables: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            57,
                                                        ),
                                                    ),
                                                    ty: 4,
                                                },
                                                ExplicitParameterDeclPattern {
                                                    pattern: 2,
                                                    variables: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            61,
                                                        ),
                                                    ),
                                                    ty: 5,
                                                },
                                            ],
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        55,
                                                    ),
                                                ),
                                                CommaToken(
                                                    TokenIdx(
                                                        59,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rpar: Ok(
                                                RightParenthesisToken(
                                                    TokenIdx(
                                                        63,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Err(
                                        Original(
                                            ExpectCurry(
                                                TokenIdx(
                                                    64,
                                                ),
                                            ),
                                        ),
                                    ),
                                    return_ty: Err(
                                        Original(
                                            ExpectOutputType(
                                                TokenIdx(
                                                    64,
                                                ),
                                            ),
                                        ),
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                64,
                                            ),
                                        ),
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::cmp::Ord`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::BoxColonList {
                                                                lbox_token_idx: TokenIdx(
                                                                    51,
                                                                ),
                                                                colon_token_idx: TokenIdx(
                                                                    52,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    1..1,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    53,
                                                                ),
                                                            },
                                                            Expr::CurrentSymbol {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    54,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                        ident_token: IdentifierToken {
                                                                            ident: `T`,
                                                                            token_idx: TokenIdx(
                                                                                43,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            Expr::ExplicitApplication {
                                                                function: 1,
                                                                argument: 2,
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::isize`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 2,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::isize`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    45,
                                                                ),
                                                                ident: `Ord`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::cmp::Ord`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    58,
                                                                ),
                                                                ident: `isize`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::isize`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    62,
                                                                ),
                                                                ident: `isize`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::isize`, `Extern`),
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
                                                                                value: 108,
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
                                                                                value: 111,
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
                                                                                value: 112,
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
                                                                    access_start: TokenIdx(
                                                                        44,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
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
                                                                    access_start: TokenIdx(
                                                                        50,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `arr`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    access_start: TokenIdx(
                                                                        57,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `low`,
                                                                        pattern_symbol_idx: 1,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    access_start: TokenIdx(
                                                                        61,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `high`,
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
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                    ),
                                                ),
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
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `low`,
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `high`,
                                                    token_idx: TokenIdx(
                                                        68,
                                                    ),
                                                    inherited_symbol_idx: 3,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `high`,
                                                    },
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
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`quick_sort::partition`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `arr`,
                                                    token_idx: TokenIdx(
                                                        75,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `low`,
                                                    token_idx: TokenIdx(
                                                        77,
                                                    ),
                                                    inherited_symbol_idx: 2,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `low`,
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `high`,
                                                    token_idx: TokenIdx(
                                                        79,
                                                    ),
                                                    inherited_symbol_idx: 3,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `high`,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                72,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 3,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        4..7,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            76,
                                                        ),
                                                        TokenIdx(
                                                            78,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 7,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                    ropd: 8,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                            ),
                                                        ),
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
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `low`,
                                                    token_idx: TokenIdx(
                                                        85,
                                                    ),
                                                    inherited_symbol_idx: 2,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `low`,
                                                    },
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 11,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        88,
                                                    ),
                                                    ropd: 12,
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 10,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        82,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        13..16,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            84,
                                                        ),
                                                        TokenIdx(
                                                            86,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        90,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                            ),
                                                        ),
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
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 18,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        96,
                                                    ),
                                                    ropd: 19,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `high`,
                                                    token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                    inherited_symbol_idx: 3,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `high`,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 17,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        92,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        20..23,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            94,
                                                        ),
                                                        TokenIdx(
                                                            98,
                                                        ),
                                                    ],
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
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::partition`, `Function`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        81,
                                                    ),
                                                    ident: `quick_sort_aux`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        91,
                                                    ),
                                                    ident: `quick_sort_aux`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::quick_sort_aux`, `Function`),
                                                        ),
                                                    ),
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
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    72,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        9,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 16,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 23,
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
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    69,
                                                                ),
                                                            ),
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
                                                                    value: 113,
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
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        kind: InheritedSymbolKind::ImplicitParameter(
                                                            Type {
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 107,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `arr`,
                                                        },
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            2,
                                                        ),
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `low`,
                                                        },
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            3,
                                                        ),
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `high`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
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
                                                            ident: `p`,
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
                                                expr: 24,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    24,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::partition`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`quick_sort::partition`, `Function`),
                                decl: FunctionDecl {
                                    path: FormPath(`quick_sort::partition`, `Function`),
                                    ast_idx: 32,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::partition`, `Function`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            113,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            114,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            115,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                ident_token: IdentifierToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        104,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 3,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            106,
                                                        ),
                                                        ident: `Ord`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            119,
                                                        ),
                                                        ident: `isize`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            123,
                                                        ),
                                                        ident: `isize`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            126,
                                                        ),
                                                        ident: `isize`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                                        value: 108,
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
                                                                        value: 111,
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
                                                                        value: 112,
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
                                                            access_start: TokenIdx(
                                                                105,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
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
                                                            access_start: TokenIdx(
                                                                111,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                118,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `low`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                122,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `high`,
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
                                    implicit_parameter_decl_list: Ok(
                                        Some(
                                            ImplicitParameterDeclList {
                                                langle: LeftAngleBracketOrLessThanToken(
                                                    TokenIdx(
                                                        103,
                                                    ),
                                                ),
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
                                                                                value: 107,
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
                                                                ColonToken(
                                                                    TokenIdx(
                                                                        105,
                                                                    ),
                                                                ),
                                                                Some(
                                                                    0,
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rangle: Ok(
                                                    RightAngleBracketToken(
                                                        TokenIdx(
                                                            107,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                    parameter_decl_list: Ok(
                                        ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    108,
                                                ),
                                            ),
                                            parameters: [
                                                ExplicitParameterDeclPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            111,
                                                        ),
                                                    ),
                                                    ty: 3,
                                                },
                                                ExplicitParameterDeclPattern {
                                                    pattern: 1,
                                                    variables: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            118,
                                                        ),
                                                    ),
                                                    ty: 4,
                                                },
                                                ExplicitParameterDeclPattern {
                                                    pattern: 2,
                                                    variables: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            122,
                                                        ),
                                                    ),
                                                    ty: 5,
                                                },
                                            ],
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        116,
                                                    ),
                                                ),
                                                CommaToken(
                                                    TokenIdx(
                                                        120,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rpar: Ok(
                                                RightParenthesisToken(
                                                    TokenIdx(
                                                        124,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                125,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 6,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                127,
                                            ),
                                        ),
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FormPath(`quick_sort::partition`, `Function`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::cmp::Ord`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::BoxColonList {
                                                                lbox_token_idx: TokenIdx(
                                                                    112,
                                                                ),
                                                                colon_token_idx: TokenIdx(
                                                                    113,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    1..1,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    114,
                                                                ),
                                                            },
                                                            Expr::CurrentSymbol {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    115,
                                                                ),
                                                                current_symbol_idx: 0,
                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                        ident_token: IdentifierToken {
                                                                            ident: `T`,
                                                                            token_idx: TokenIdx(
                                                                                104,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            Expr::ExplicitApplication {
                                                                function: 1,
                                                                argument: 2,
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::isize`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 2,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::isize`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 3,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::isize`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    106,
                                                                ),
                                                                ident: `Ord`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::cmp::Ord`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    119,
                                                                ),
                                                                ident: `isize`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::isize`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    123,
                                                                ),
                                                                ident: `isize`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::isize`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    126,
                                                                ),
                                                                ident: `isize`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::isize`, `Extern`),
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
                                                                                value: 108,
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
                                                                                value: 111,
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
                                                                                value: 112,
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
                                                                    access_start: TokenIdx(
                                                                        105,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
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
                                                                    access_start: TokenIdx(
                                                                        111,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `arr`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    access_start: TokenIdx(
                                                                        118,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `low`,
                                                                        pattern_symbol_idx: 1,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    access_start: TokenIdx(
                                                                        122,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `high`,
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
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::partition`, `Function`),
                                                    ),
                                                ),
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
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `high`,
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::usize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                130,
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
                                                Expr::BinaryOpn {
                                                    lopd: 2,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                    ropd: 3,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `low`,
                                                    token_idx: TokenIdx(
                                                        138,
                                                    ),
                                                    inherited_symbol_idx: 2,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `low`,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        140,
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                137,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::BinaryOpn {
                                                    lopd: 5,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        139,
                                                    ),
                                                    ropd: 6,
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 7,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        137,
                                                    ),
                                                    ropd: 8,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                144,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::InheritedSymbol {
                                                    ident: `high`,
                                                    token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                    inherited_symbol_idx: 3,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `high`,
                                                    },
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 10,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        144,
                                                    ),
                                                    ropd: 11,
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
                                                    lopd: 14,
                                                    opr: Assign(
                                                        Some(
                                                            Add,
                                                        ),
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        150,
                                                    ),
                                                    ropd: 15,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `arr`,
                                                    token_idx: TokenIdx(
                                                        153,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::usize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 18,
                                                    opr: As,
                                                    opr_token_idx: TokenIdx(
                                                        156,
                                                    ),
                                                    ropd: 19,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `arr`,
                                                    token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
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
                                                Expr::IndexOrComposeWithList {
                                                    owner: 17,
                                                    lbox_token_idx: TokenIdx(
                                                        154,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        20..21,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        158,
                                                    ),
                                                },
                                                Expr::IndexOrComposeWithList {
                                                    owner: 21,
                                                    lbox_token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        22..23,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        163,
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 23,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        159,
                                                    ),
                                                    ropd: 24,
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
                                                    lopd: 26,
                                                    opr: Assign(
                                                        Some(
                                                            Add,
                                                        ),
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        166,
                                                    ),
                                                    ropd: 27,
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
                                                    lopd: 29,
                                                    opr: Assign(
                                                        Some(
                                                            Sub,
                                                        ),
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        169,
                                                    ),
                                                    ropd: 30,
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
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::usize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 35,
                                                    opr: As,
                                                    opr_token_idx: TokenIdx(
                                                        179,
                                                    ),
                                                    ropd: 36,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `arr`,
                                                    token_idx: TokenIdx(
                                                        183,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
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
                                                Expr::IndexOrComposeWithList {
                                                    owner: 34,
                                                    lbox_token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        37..38,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                },
                                                Expr::IndexOrComposeWithList {
                                                    owner: 38,
                                                    lbox_token_idx: TokenIdx(
                                                        184,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        39..40,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        186,
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 32,
                                                    opr: Comparison(
                                                        Geq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        173,
                                                    ),
                                                    ropd: 33,
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 40,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        182,
                                                    ),
                                                    ropd: 41,
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 42,
                                                    opr: ShortCircuitLogic(
                                                        And,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        175,
                                                    ),
                                                    ropd: 43,
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
                                                    lopd: 45,
                                                    opr: Assign(
                                                        Some(
                                                            Sub,
                                                        ),
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        189,
                                                    ),
                                                    ropd: 46,
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
                                                    lopd: 48,
                                                    opr: Comparison(
                                                        Geq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        193,
                                                    ),
                                                    ropd: 49,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `arr`,
                                                    token_idx: TokenIdx(
                                                        199,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::usize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::usize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 52,
                                                    opr: As,
                                                    opr_token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                    ropd: 53,
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 54,
                                                    opr: As,
                                                    opr_token_idx: TokenIdx(
                                                        208,
                                                    ),
                                                    ropd: 55,
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 51,
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
                                                        56..58,
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
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `arr`,
                                                    },
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 5,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::usize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 6,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::usize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 60,
                                                    opr: As,
                                                    opr_token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    ropd: 61,
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 62,
                                                    opr: As,
                                                    opr_token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                    ropd: 63,
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 59,
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
                                                        64..66,
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
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        133,
                                                    ),
                                                    ident: `usize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                    ident: `usize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        180,
                                                    ),
                                                    ident: `usize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                    ident: `usize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                    ident: `usize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                    ident: `usize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                    ident: `usize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Eval {
                                                    expr_idx: 28,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 47,
                                                },
                                                Stmt::Break {
                                                    break_token: BreakToken {
                                                        token_idx: TokenIdx(
                                                            196,
                                                        ),
                                                    },
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 58,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 16,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            152,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        25,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolColonToken(
                                                            TokenIdx(
                                                                164,
                                                            ),
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 31,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            171,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        44,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolColonToken(
                                                            TokenIdx(
                                                                187,
                                                            ),
                                                        ),
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
                                                            50,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    195,
                                                                ),
                                                            ),
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
                                                                EolColonToken(
                                                                    TokenIdx(
                                                                        198,
                                                                    ),
                                                                ),
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
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    130,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        4,
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
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    137,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        9,
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
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    144,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        12,
                                                    ),
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            146,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        13,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolColonToken(
                                                            TokenIdx(
                                                                148,
                                                            ),
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            4..9,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 66,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 67,
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
                                                                    value: 115,
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
                                                                    value: 116,
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
                                                                    value: 117,
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
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        kind: InheritedSymbolKind::ImplicitParameter(
                                                            Type {
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 107,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `arr`,
                                                        },
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            2,
                                                        ),
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `low`,
                                                        },
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            3,
                                                        ),
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `high`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
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
                                                            ident: `pivot`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
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
                                                            ident: `store_index`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
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
                                                            ident: `last_index`,
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
                                                expr: 68,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    68,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                                decl: FeatureDecl {
                                    path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                                    ast_idx: 34,
                                    curry_token: Err(
                                        Original(
                                            ExpectCurry(
                                                TokenIdx(
                                                    228,
                                                ),
                                            ),
                                        ),
                                    ),
                                    return_ty: Err(
                                        Original(
                                            ExpectOutputType(
                                                TokenIdx(
                                                    228,
                                                ),
                                            ),
                                        ),
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                228,
                                            ),
                                        ),
                                    ),
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
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
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                                                    ),
                                                ),
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
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                232,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::BoxList {
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
                                                Expr::BinaryOpn {
                                                    lopd: 11,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        232,
                                                    ),
                                                    ropd: 12,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`quick_sort::quick_sort`, `Function`),
                                                            ),
                                                        ),
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
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 14,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        256,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        15..16,
                                                    ),
                                                    commas: [],
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
                                                    opd: 17,
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
                                                Expr::BoxList {
                                                    lbox_token_idx: TokenIdx(
                                                        262,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        18..28,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        283,
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 28,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        261,
                                                    ),
                                                    ropd: 29,
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
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::quick_sort`, `Function`),
                                                        ),
                                                    ),
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
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    232,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        13,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 16,
                                                },
                                                Stmt::Assert {
                                                    assert_token: AssertToken {
                                                        token_idx: TokenIdx(
                                                            259,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        30,
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
                                                                    value: 121,
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
                                                            ident: `v`,
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
                                                expr: 31,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    31,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                                decl: FeatureDecl {
                                    path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                                    ast_idx: 36,
                                    curry_token: Err(
                                        Original(
                                            ExpectCurry(
                                                TokenIdx(
                                                    288,
                                                ),
                                            ),
                                        ),
                                    ),
                                    return_ty: Err(
                                        Original(
                                            ExpectOutputType(
                                                TokenIdx(
                                                    288,
                                                ),
                                            ),
                                        ),
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                288,
                                            ),
                                        ),
                                    ),
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
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
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                                                    ),
                                                ),
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
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                292,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::BoxList {
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
                                                Expr::BinaryOpn {
                                                    lopd: 6,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        292,
                                                    ),
                                                    ropd: 7,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`quick_sort::quick_sort`, `Function`),
                                                            ),
                                                        ),
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
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 9,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        307,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        10..11,
                                                    ),
                                                    commas: [],
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
                                                Expr::BoxList {
                                                    lbox_token_idx: TokenIdx(
                                                        313,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        12..18,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        325,
                                                    ),
                                                },
                                                Expr::BinaryOpn {
                                                    lopd: 18,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        312,
                                                    ),
                                                    ropd: 19,
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
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`quick_sort::quick_sort`, `Function`),
                                                        ),
                                                    ),
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
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    292,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        8,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 11,
                                                },
                                                Stmt::Assert {
                                                    assert_token: AssertToken {
                                                        token_idx: TokenIdx(
                                                            310,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        20,
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
                                                                    value: 123,
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
                                                            ident: `strs`,
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
                                                expr: 21,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    21,
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)