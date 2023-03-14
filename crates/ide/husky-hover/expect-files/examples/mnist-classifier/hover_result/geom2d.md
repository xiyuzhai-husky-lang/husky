Ok(
    [
        (
            TokenIdx(
                0,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 0;\n\ntoken = Token::Attr(\n    AttributeKeyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 0,
                                },
                                end: Position {
                                    line: 0,
                                    character: 3,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                39,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 39;\n\ntoken = Token::Ident(\n    `f32`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Type(\n                TypePath(`core::num::f32`, `Extern`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 32,
                                },
                                end: Position {
                                    line: 7,
                                    character: 35,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                78,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 78;\n\ntoken = Token::Ident(\n    `other`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {\n        ident: `other`,\n    },\n    expr_region: ExprRegion {\n        data: ExprRegionData {\n            parent: Some(\n                ExprRegion {\n                    data: ExprRegionData {\n                        parent: Some(\n                            ExprRegion {\n                                data: ExprRegionData {\n                                    parent: None,\n                                    path: RegionPath::Decl(\n                                        DeclRegionPath::Impl(\n                                            ImplBlockId::Type(\n                                                TypeImplBlockId {\n                                                    module: `mnist_classifier::geom2d`,\n                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                                    disambiguator: 0,\n                                                },\n                                            ),\n                                        ),\n                                    ),\n                                    expr_arena: Arena {\n                                        data: [\n                                            Expr::EntityPath {\n                                                entity_path_expr: 0,\n                                                path: Some(\n                                                    EntityPath::ModuleItem(\n                                                        ModuleItemPath::Type(\n                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                                        ),\n                                                    ),\n                                                ),\n                                            },\n                                        ],\n                                    },\n                                    entity_path_expr_arena: Arena {\n                                        data: [\n                                            EntityPathExpr::Root {\n                                                token_idx: TokenIdx(\n                                                    14,\n                                                ),\n                                                ident: `Point2d`,\n                                                entity_path: EntityPath::ModuleItem(\n                                                    ModuleItemPath::Type(\n                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                                    ),\n                                                ),\n                                            },\n                                        ],\n                                    },\n                                    stmt_arena: Arena {\n                                        data: [],\n                                    },\n                                    pattern_expr_region: PatternExprRegion {\n                                        pattern_expr_arena: Arena {\n                                            data: [],\n                                        },\n                                        pattern_infos: [],\n                                        pattern_symbol_maps: [],\n                                        pattern_symbol_arena: Arena {\n                                            data: [],\n                                        },\n                                    },\n                                    symbol_region: SymbolRegion {\n                                        inherited_symbol_arena: Arena {\n                                            data: [],\n                                        },\n                                        current_symbol_arena: Arena {\n                                            data: [],\n                                        },\n                                        allow_self_type: True,\n                                        allow_self_value: False,\n                                        pattern_ty_constraints: [],\n                                    },\n                                    roots: [\n                                        ExprRoot {\n                                            kind: SelfType,\n                                            expr: 0,\n                                        },\n                                    ],\n                                },\n                            },\n                        ),\n                        path: RegionPath::Decl(\n                            DeclRegionPath::AssociatedItem(\n                                AssociatedItemId {\n                                    impl_block_id: ImplBlockId::Type(\n                                        TypeImplBlockId {\n                                            module: `mnist_classifier::geom2d`,\n                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                            disambiguator: 0,\n                                        },\n                                    ),\n                                    ident: `to`,\n                                },\n                            ),\n                        ),\n                        expr_arena: Arena {\n                            data: [\n                                Expr::EntityPath {\n                                    entity_path_expr: 0,\n                                    path: Some(\n                                        EntityPath::ModuleItem(\n                                            ModuleItemPath::Type(\n                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                            ),\n                                        ),\n                                    ),\n                                },\n                                Expr::EntityPath {\n                                    entity_path_expr: 1,\n                                    path: Some(\n                                        EntityPath::ModuleItem(\n                                            ModuleItemPath::Type(\n                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),\n                                            ),\n                                        ),\n                                    ),\n                                },\n                            ],\n                        },\n                        entity_path_expr_arena: Arena {\n                            data: [\n                                EntityPathExpr::Root {\n                                    token_idx: TokenIdx(\n                                        71,\n                                    ),\n                                    ident: `Point2d`,\n                                    entity_path: EntityPath::ModuleItem(\n                                        ModuleItemPath::Type(\n                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                        ),\n                                    ),\n                                },\n                                EntityPathExpr::Root {\n                                    token_idx: TokenIdx(\n                                        74,\n                                    ),\n                                    ident: `Vector2d`,\n                                    entity_path: EntityPath::ModuleItem(\n                                        ModuleItemPath::Type(\n                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),\n                                        ),\n                                    ),\n                                },\n                            ],\n                        },\n                        stmt_arena: Arena {\n                            data: [],\n                        },\n                        pattern_expr_region: PatternExprRegion {\n                            pattern_expr_arena: Arena {\n                                data: [\n                                    PatternExpr::Ident {\n                                        ident_token: IdentToken {\n                                            ident: `other`,\n                                            token_idx: TokenIdx(\n                                                69,\n                                            ),\n                                        },\n                                        liason: None,\n                                    },\n                                ],\n                            },\n                            pattern_infos: [\n                                Parameter,\n                            ],\n                            pattern_symbol_maps: [\n                                [\n                                    (\n                                        `other`,\n                                        0,\n                                    ),\n                                ],\n                            ],\n                            pattern_symbol_arena: Arena {\n                                data: [\n                                    PatternSymbol::Atom(\n                                        0,\n                                    ),\n                                ],\n                            },\n                        },\n                        symbol_region: SymbolRegion {\n                            inherited_symbol_arena: Arena {\n                                data: [],\n                            },\n                            current_symbol_arena: Arena {\n                                data: [\n                                    CurrentSymbol {\n                                        access_start: TokenIdx(\n                                            70,\n                                        ),\n                                        access_end: None,\n                                        variant: CurrentSymbolVariant::RegularParameter {\n                                            ident: `other`,\n                                            pattern_symbol_idx: 0,\n                                        },\n                                    },\n                                ],\n                            },\n                            allow_self_type: True,\n                            allow_self_value: True,\n                            pattern_ty_constraints: [\n                                RegularParameter {\n                                    pattern: 0,\n                                    ty: 0,\n                                },\n                            ],\n                        },\n                        roots: [\n                            ExprRoot {\n                                kind: ReturnType,\n                                expr: 1,\n                            },\n                        ],\n                    },\n                },\n            ),\n            path: RegionPath::Defn(\n                DefnRegionPath::AssociatedItem(\n                    AssociatedItemId {\n                        impl_block_id: ImplBlockId::Type(\n                            TypeImplBlockId {\n                                module: `mnist_classifier::geom2d`,\n                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                disambiguator: 0,\n                            },\n                        ),\n                        ident: `to`,\n                    },\n                ),\n            ),\n            expr_arena: Arena {\n                data: [\n                    Expr::EntityPath {\n                        entity_path_expr: 0,\n                        path: Some(\n                            EntityPath::ModuleItem(\n                                ModuleItemPath::Type(\n                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),\n                                ),\n                            ),\n                        ),\n                    },\n                    Expr::InheritedSymbol {\n                        ident: `other`,\n                        token_idx: TokenIdx(\n                            78,\n                        ),\n                        inherited_symbol_idx: 0,\n                        inherited_symbol_kind: InheritedSymbolKind::RegularParameter {\n                            ident: `other`,\n                        },\n                    },\n                    Expr::SelfValue(\n                        TokenIdx(\n                            82,\n                        ),\n                    ),\n                    Expr::Field {\n                        owner: 1,\n                        dot_token_idx: TokenIdx(\n                            79,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `x`,\n                            token_idx: TokenIdx(\n                                80,\n                            ),\n                        },\n                    },\n                    Expr::Field {\n                        owner: 2,\n                        dot_token_idx: TokenIdx(\n                            83,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `x`,\n                            token_idx: TokenIdx(\n                                84,\n                            ),\n                        },\n                    },\n                    Expr::InheritedSymbol {\n                        ident: `other`,\n                        token_idx: TokenIdx(\n                            86,\n                        ),\n                        inherited_symbol_idx: 0,\n                        inherited_symbol_kind: InheritedSymbolKind::RegularParameter {\n                            ident: `other`,\n                        },\n                    },\n                    Expr::SelfValue(\n                        TokenIdx(\n                            90,\n                        ),\n                    ),\n                    Expr::Field {\n                        owner: 5,\n                        dot_token_idx: TokenIdx(\n                            87,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `y`,\n                            token_idx: TokenIdx(\n                                88,\n                            ),\n                        },\n                    },\n                    Expr::Field {\n                        owner: 6,\n                        dot_token_idx: TokenIdx(\n                            91,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `y`,\n                            token_idx: TokenIdx(\n                                92,\n                            ),\n                        },\n                    },\n                    Expr::Binary {\n                        lopd: 3,\n                        opr: Closed(\n                            Sub,\n                        ),\n                        opr_token_idx: TokenIdx(\n                            81,\n                        ),\n                        ropd: 4,\n                    },\n                    Expr::Binary {\n                        lopd: 7,\n                        opr: Closed(\n                            Sub,\n                        ),\n                        opr_token_idx: TokenIdx(\n                            89,\n                        ),\n                        ropd: 8,\n                    },\n                    Expr::ExplicitApplicationOrRitchieCall {\n                        function: 0,\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            77,\n                        ),\n                        items: ArenaIdxRange(\n                            9..11,\n                        ),\n                        commas: [\n                            TokenIdx(\n                                85,\n                            ),\n                        ],\n                        rpar_token_idx: TokenIdx(\n                            93,\n                        ),\n                    },\n                    Expr::Block {\n                        stmts: ArenaIdxRange(\n                            0..1,\n                        ),\n                    },\n                ],\n            },\n            entity_path_expr_arena: Arena {\n                data: [\n                    EntityPathExpr::Root {\n                        token_idx: TokenIdx(\n                            76,\n                        ),\n                        ident: `Vector2d`,\n                        entity_path: EntityPath::ModuleItem(\n                            ModuleItemPath::Type(\n                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),\n                            ),\n                        ),\n                    },\n                ],\n            },\n            stmt_arena: Arena {\n                data: [\n                    Stmt::Eval {\n                        expr_idx: 11,\n                    },\n                ],\n            },\n            pattern_expr_region: PatternExprRegion {\n                pattern_expr_arena: Arena {\n                    data: [],\n                },\n                pattern_infos: [],\n                pattern_symbol_maps: [],\n                pattern_symbol_arena: Arena {\n                    data: [],\n                },\n            },\n            symbol_region: SymbolRegion {\n                inherited_symbol_arena: Arena {\n                    data: [\n                        InheritedSymbol {\n                            parent_symbol_idx: Current(\n                                0,\n                            ),\n                            kind: InheritedSymbolKind::RegularParameter {\n                                ident: `other`,\n                            },\n                        },\n                    ],\n                },\n                current_symbol_arena: Arena {\n                    data: [],\n                },\n                allow_self_type: True,\n                allow_self_value: True,\n                pattern_ty_constraints: [],\n            },\n            roots: [\n                ExprRoot {\n                    kind: BlockExpr,\n                    expr: 12,\n                },\n            ],\n        },\n    },\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        0,\n    ),\n    kind: InheritedSymbolKind::RegularParameter {\n        ident: `other`,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 17,
                                },
                                end: Position {
                                    line: 13,
                                    character: 22,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                117,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 117;\n\ntoken = Token::Punctuation(\n    Punctuation::Ket(\n        Par,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 16,
                                    character: 42,
                                },
                                end: Position {
                                    line: 16,
                                    character: 43,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                156,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 156;\n\ntoken = Token::Keyword(\n    Keyword::Type(\n        Struct,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 26,
                                    character: 4,
                                },
                                end: Position {
                                    line: 26,
                                    character: 10,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                195,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 195;\n\ntoken = Token::Ident(\n    `Vector2d`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Type(\n                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 35,
                                    character: 23,
                                },
                                end: Position {
                                    line: 35,
                                    character: 31,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                234,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 234;\n\ntoken = Token::Keyword(\n    Keyword::Pronoun(\n        SelfValue,\n    ),\n);\n\ntoken_info = TokenInfo::SelfValue;\n\nself value\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 39,
                                    character: 27,
                                },
                                end: Position {
                                    line: 39,
                                    character: 31,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                273,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 273;\n\ntoken = Token::Punctuation(\n    Punctuation::Bra(\n        Par,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 44,
                                    character: 14,
                                },
                                end: Position {
                                    line: 44,
                                    character: 15,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                312,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 312;\n\ntoken = Token::Ident(\n    `x`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 48,
                                    character: 30,
                                },
                                end: Position {
                                    line: 48,
                                    character: 31,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                351,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 351;\n\ntoken = Token::Ident(\n    `f32`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Type(\n                TypePath(`core::num::f32`, `Extern`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 55,
                                    character: 30,
                                },
                                end: Position {
                                    line: 55,
                                    character: 33,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                390,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 390;\n\ntoken = Token::Ident(\n    `Vector2d`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Type(\n                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 64,
                                    character: 25,
                                },
                                end: Position {
                                    line: 64,
                                    character: 33,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                429,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 429;\n\ntoken = Token::Ident(\n    `dot`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 69,
                                    character: 30,
                                },
                                end: Position {
                                    line: 69,
                                    character: 33,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                468,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 468;\n\ntoken = Token::Ident(\n    `rotation_direction_to`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 76,
                                    character: 34,
                                },
                                end: Position {
                                    line: 76,
                                    character: 55,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                507,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 507;\n\ntoken = Token::Ident(\n    `ClosedRange`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 85,
                                    character: 31,
                                },
                                end: Position {
                                    line: 85,
                                    character: 42,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                546,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 546;\n\ntoken = Token::Punctuation(\n    Punctuation::Eq,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 89,
                                    character: 20,
                                },
                                end: Position {
                                    line: 89,
                                    character: 21,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                585,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 585;\n\ntoken = Token::Punctuation(\n    Punctuation::Bra(\n        Par,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 97,
                                    character: 8,
                                },
                                end: Position {
                                    line: 97,
                                    character: 9,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                624,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 624;\n\ntoken = Token::Keyword(\n    Keyword::Pronoun(\n        SelfValue,\n    ),\n);\n\ntoken_info = TokenInfo::SelfValue;\n\nself value\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 107,
                                    character: 12,
                                },
                                end: Position {
                                    line: 107,
                                    character: 16,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                663,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 663;\n\ntoken = Token::Ident(\n    `relative_point`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 113,
                                    character: 24,
                                },
                                end: Position {
                                    line: 113,
                                    character: 38,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                702,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 702;\n\ntoken = Token::Punctuation(\n    Punctuation::Dot,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 121,
                                    character: 12,
                                },
                                end: Position {
                                    line: 121,
                                    character: 13,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                741,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 741;\n\ntoken = Token::Punctuation(\n    Punctuation::Comma,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 131,
                                    character: 23,
                                },
                                end: Position {
                                    line: 131,
                                    character: 24,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                780,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 780;\n\ntoken = Token::Punctuation(\n    Punctuation::Dot,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 142,
                                    character: 19,
                                },
                                end: Position {
                                    line: 142,
                                    character: 20,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
    ],
)