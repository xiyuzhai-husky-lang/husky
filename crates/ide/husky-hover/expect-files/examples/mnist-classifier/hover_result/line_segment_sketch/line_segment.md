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
                                value: "Other\ntoken_idx = 0;\n\ntoken = Token::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                6,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 6;\n\ntoken = Token::Attr(\n    AttributeKeyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 0,
                                },
                                end: Position {
                                    line: 2,
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
                12,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 12;\n\ntoken = Token::Ident(\n    `Point2d`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Type(\n                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 11,
                                },
                                end: Position {
                                    line: 3,
                                    character: 18,
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
                18,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 18;\n\ntoken = Token::Punctuation(\n    Punctuation::Ket(\n        Curl,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 5,
                                    character: 0,
                                },
                                end: Position {
                                    line: 5,
                                    character: 1,
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
                24,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 24;\n\ntoken = Token::Punctuation(\n    Punctuation::Bra(\n        Par,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 21,
                                },
                                end: Position {
                                    line: 8,
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
                30,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 30;\n\ntoken = Token::Punctuation(\n    Punctuation::Dot,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 12,
                                },
                                end: Position {
                                    line: 9,
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
                36,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 36;\n\ntoken = Token::Punctuation(\n    Punctuation::Dot,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 26,
                                },
                                end: Position {
                                    line: 9,
                                    character: 27,
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
                42,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 42;\n\ntoken = Token::Ident(\n    `pt`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSymbolKind::Parameter {\n        pattern_symbol_idx: 0,\n    },\n    expr_region: ExprRegion {\n        data: ExprRegionData {\n            parent: Some(\n                ExprRegion {\n                    data: ExprRegionData {\n                        parent: None,\n                        path: RegionPath::Decl(\n                            DeclRegionPath::Impl(\n                                ImplBlockId::Type(\n                                    TypeImplBlockId {\n                                        module: `mnist_classifier::line_segment_sketch::line_segment`,\n                                        ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                        disambiguator: 0,\n                                    },\n                                ),\n                            ),\n                        ),\n                        expr_arena: Arena {\n                            data: [\n                                Expr::EntityPath {\n                                    entity_path_expr: 0,\n                                    path: Some(\n                                        EntityPath::ModuleItem(\n                                            ModuleItemPath::Type(\n                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                            ),\n                                        ),\n                                    ),\n                                },\n                            ],\n                        },\n                        entity_path_expr_arena: Arena {\n                            data: [\n                                EntityPathExpr::Root {\n                                    token_idx: TokenIdx(\n                                        20,\n                                    ),\n                                    ident: `LineSegment`,\n                                    entity_path: EntityPath::ModuleItem(\n                                        ModuleItemPath::Type(\n                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                        ),\n                                    ),\n                                },\n                            ],\n                        },\n                        stmt_arena: Arena {\n                            data: [],\n                        },\n                        pattern_expr_region: PatternExprRegion {\n                            pattern_expr_arena: Arena {\n                                data: [],\n                            },\n                            pattern_infos: [],\n                            pattern_symbol_maps: [],\n                            pattern_symbol_arena: Arena {\n                                data: [],\n                            },\n                        },\n                        symbol_region: SymbolRegion {\n                            inherited_symbol_arena: Arena {\n                                data: [],\n                            },\n                            current_symbol_arena: Arena {\n                                data: [],\n                            },\n                            allow_self_type: True,\n                            allow_self_value: False,\n                            pattern_ty_constraints: [],\n                        },\n                        roots: [\n                            ExprRoot {\n                                kind: SelfType,\n                                expr: 0,\n                            },\n                        ],\n                    },\n                },\n            ),\n            path: RegionPath::Decl(\n                DeclRegionPath::AssociatedItem(\n                    AssociatedItemId {\n                        impl_block_id: ImplBlockId::Type(\n                            TypeImplBlockId {\n                                module: `mnist_classifier::line_segment_sketch::line_segment`,\n                                ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                disambiguator: 0,\n                            },\n                        ),\n                        ident: `dist_to_point`,\n                    },\n                ),\n            ),\n            expr_arena: Arena {\n                data: [\n                    Expr::EntityPath {\n                        entity_path_expr: 0,\n                        path: Some(\n                            EntityPath::ModuleItem(\n                                ModuleItemPath::Type(\n                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                ),\n                            ),\n                        ),\n                    },\n                    Expr::EntityPath {\n                        entity_path_expr: 1,\n                        path: Some(\n                            EntityPath::ModuleItem(\n                                ModuleItemPath::Type(\n                                    TypePath(`core::num::f32`, `Extern`),\n                                ),\n                            ),\n                        ),\n                    },\n                ],\n            },\n            entity_path_expr_arena: Arena {\n                data: [\n                    EntityPathExpr::Root {\n                        token_idx: TokenIdx(\n                            44,\n                        ),\n                        ident: `Point2d`,\n                        entity_path: EntityPath::ModuleItem(\n                            ModuleItemPath::Type(\n                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                            ),\n                        ),\n                    },\n                    EntityPathExpr::Root {\n                        token_idx: TokenIdx(\n                            47,\n                        ),\n                        ident: `f32`,\n                        entity_path: EntityPath::ModuleItem(\n                            ModuleItemPath::Type(\n                                TypePath(`core::num::f32`, `Extern`),\n                            ),\n                        ),\n                    },\n                ],\n            },\n            stmt_arena: Arena {\n                data: [],\n            },\n            pattern_expr_region: PatternExprRegion {\n                pattern_expr_arena: Arena {\n                    data: [\n                        PatternExpr::Ident {\n                            ident_token: IdentToken {\n                                ident: `pt`,\n                                token_idx: TokenIdx(\n                                    42,\n                                ),\n                            },\n                            liason: None,\n                        },\n                    ],\n                },\n                pattern_infos: [\n                    Parameter,\n                ],\n                pattern_symbol_maps: [\n                    [\n                        (\n                            `pt`,\n                            0,\n                        ),\n                    ],\n                ],\n                pattern_symbol_arena: Arena {\n                    data: [\n                        PatternSymbol::Atom(\n                            0,\n                        ),\n                    ],\n                },\n            },\n            symbol_region: SymbolRegion {\n                inherited_symbol_arena: Arena {\n                    data: [],\n                },\n                current_symbol_arena: Arena {\n                    data: [\n                        CurrentSymbol {\n                            access_start: TokenIdx(\n                                43,\n                            ),\n                            access_end: None,\n                            variant: CurrentSymbolVariant::RegularParameter {\n                                ident: `pt`,\n                                pattern_symbol_idx: 0,\n                            },\n                        },\n                    ],\n                },\n                allow_self_type: True,\n                allow_self_value: True,\n                pattern_ty_constraints: [\n                    RegularParameter {\n                        pattern: 0,\n                        ty: 0,\n                    },\n                ],\n            },\n            roots: [\n                ExprRoot {\n                    kind: ReturnType,\n                    expr: 1,\n                },\n            ],\n        },\n    },\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        43,\n    ),\n    access_end: None,\n    variant: CurrentSymbolVariant::RegularParameter {\n        ident: `pt`,\n        pattern_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 23,
                                },
                                end: Position {
                                    line: 11,
                                    character: 25,
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
                48,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 48;\n\ntoken = Token::Punctuation(\n    Punctuation::Colon,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 42,
                                },
                                end: Position {
                                    line: 11,
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
                54,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 54;\n\ntoken = Token::Ident(\n    `displacement`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 22,
                                },
                                end: Position {
                                    line: 12,
                                    character: 34,
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
                60,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 60;\n\ntoken = Token::Keyword(\n    Keyword::Pronoun(\n        SelfValue,\n    ),\n);\n\ntoken_info = TokenInfo::SelfValue;\n\nself value\n",
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
                66,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 66;\n\ntoken = Token::Ident(\n    `pt`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {\n        ident: `pt`,\n    },\n    expr_region: ExprRegion {\n        data: ExprRegionData {\n            parent: Some(\n                ExprRegion {\n                    data: ExprRegionData {\n                        parent: Some(\n                            ExprRegion {\n                                data: ExprRegionData {\n                                    parent: None,\n                                    path: RegionPath::Decl(\n                                        DeclRegionPath::Impl(\n                                            ImplBlockId::Type(\n                                                TypeImplBlockId {\n                                                    module: `mnist_classifier::line_segment_sketch::line_segment`,\n                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                                    disambiguator: 0,\n                                                },\n                                            ),\n                                        ),\n                                    ),\n                                    expr_arena: Arena {\n                                        data: [\n                                            Expr::EntityPath {\n                                                entity_path_expr: 0,\n                                                path: Some(\n                                                    EntityPath::ModuleItem(\n                                                        ModuleItemPath::Type(\n                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                                        ),\n                                                    ),\n                                                ),\n                                            },\n                                        ],\n                                    },\n                                    entity_path_expr_arena: Arena {\n                                        data: [\n                                            EntityPathExpr::Root {\n                                                token_idx: TokenIdx(\n                                                    20,\n                                                ),\n                                                ident: `LineSegment`,\n                                                entity_path: EntityPath::ModuleItem(\n                                                    ModuleItemPath::Type(\n                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                                    ),\n                                                ),\n                                            },\n                                        ],\n                                    },\n                                    stmt_arena: Arena {\n                                        data: [],\n                                    },\n                                    pattern_expr_region: PatternExprRegion {\n                                        pattern_expr_arena: Arena {\n                                            data: [],\n                                        },\n                                        pattern_infos: [],\n                                        pattern_symbol_maps: [],\n                                        pattern_symbol_arena: Arena {\n                                            data: [],\n                                        },\n                                    },\n                                    symbol_region: SymbolRegion {\n                                        inherited_symbol_arena: Arena {\n                                            data: [],\n                                        },\n                                        current_symbol_arena: Arena {\n                                            data: [],\n                                        },\n                                        allow_self_type: True,\n                                        allow_self_value: False,\n                                        pattern_ty_constraints: [],\n                                    },\n                                    roots: [\n                                        ExprRoot {\n                                            kind: SelfType,\n                                            expr: 0,\n                                        },\n                                    ],\n                                },\n                            },\n                        ),\n                        path: RegionPath::Decl(\n                            DeclRegionPath::AssociatedItem(\n                                AssociatedItemId {\n                                    impl_block_id: ImplBlockId::Type(\n                                        TypeImplBlockId {\n                                            module: `mnist_classifier::line_segment_sketch::line_segment`,\n                                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                            disambiguator: 0,\n                                        },\n                                    ),\n                                    ident: `dist_to_point`,\n                                },\n                            ),\n                        ),\n                        expr_arena: Arena {\n                            data: [\n                                Expr::EntityPath {\n                                    entity_path_expr: 0,\n                                    path: Some(\n                                        EntityPath::ModuleItem(\n                                            ModuleItemPath::Type(\n                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                            ),\n                                        ),\n                                    ),\n                                },\n                                Expr::EntityPath {\n                                    entity_path_expr: 1,\n                                    path: Some(\n                                        EntityPath::ModuleItem(\n                                            ModuleItemPath::Type(\n                                                TypePath(`core::num::f32`, `Extern`),\n                                            ),\n                                        ),\n                                    ),\n                                },\n                            ],\n                        },\n                        entity_path_expr_arena: Arena {\n                            data: [\n                                EntityPathExpr::Root {\n                                    token_idx: TokenIdx(\n                                        44,\n                                    ),\n                                    ident: `Point2d`,\n                                    entity_path: EntityPath::ModuleItem(\n                                        ModuleItemPath::Type(\n                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                        ),\n                                    ),\n                                },\n                                EntityPathExpr::Root {\n                                    token_idx: TokenIdx(\n                                        47,\n                                    ),\n                                    ident: `f32`,\n                                    entity_path: EntityPath::ModuleItem(\n                                        ModuleItemPath::Type(\n                                            TypePath(`core::num::f32`, `Extern`),\n                                        ),\n                                    ),\n                                },\n                            ],\n                        },\n                        stmt_arena: Arena {\n                            data: [],\n                        },\n                        pattern_expr_region: PatternExprRegion {\n                            pattern_expr_arena: Arena {\n                                data: [\n                                    PatternExpr::Ident {\n                                        ident_token: IdentToken {\n                                            ident: `pt`,\n                                            token_idx: TokenIdx(\n                                                42,\n                                            ),\n                                        },\n                                        liason: None,\n                                    },\n                                ],\n                            },\n                            pattern_infos: [\n                                Parameter,\n                            ],\n                            pattern_symbol_maps: [\n                                [\n                                    (\n                                        `pt`,\n                                        0,\n                                    ),\n                                ],\n                            ],\n                            pattern_symbol_arena: Arena {\n                                data: [\n                                    PatternSymbol::Atom(\n                                        0,\n                                    ),\n                                ],\n                            },\n                        },\n                        symbol_region: SymbolRegion {\n                            inherited_symbol_arena: Arena {\n                                data: [],\n                            },\n                            current_symbol_arena: Arena {\n                                data: [\n                                    CurrentSymbol {\n                                        access_start: TokenIdx(\n                                            43,\n                                        ),\n                                        access_end: None,\n                                        variant: CurrentSymbolVariant::RegularParameter {\n                                            ident: `pt`,\n                                            pattern_symbol_idx: 0,\n                                        },\n                                    },\n                                ],\n                            },\n                            allow_self_type: True,\n                            allow_self_value: True,\n                            pattern_ty_constraints: [\n                                RegularParameter {\n                                    pattern: 0,\n                                    ty: 0,\n                                },\n                            ],\n                        },\n                        roots: [\n                            ExprRoot {\n                                kind: ReturnType,\n                                expr: 1,\n                            },\n                        ],\n                    },\n                },\n            ),\n            path: RegionPath::Defn(\n                DefnRegionPath::AssociatedItem(\n                    AssociatedItemId {\n                        impl_block_id: ImplBlockId::Type(\n                            TypeImplBlockId {\n                                module: `mnist_classifier::line_segment_sketch::line_segment`,\n                                ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                disambiguator: 0,\n                            },\n                        ),\n                        ident: `dist_to_point`,\n                    },\n                ),\n            ),\n            expr_arena: Arena {\n                data: [\n                    Expr::SelfValue(\n                        TokenIdx(\n                            52,\n                        ),\n                    ),\n                    Expr::MethodCall {\n                        self_argument: 0,\n                        dot_token_idx: TokenIdx(\n                            53,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `displacement`,\n                            token_idx: TokenIdx(\n                                54,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            55,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            1..1,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            56,\n                        ),\n                    },\n                    Expr::SelfValue(\n                        TokenIdx(\n                            60,\n                        ),\n                    ),\n                    Expr::Field {\n                        owner: 2,\n                        dot_token_idx: TokenIdx(\n                            61,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `start`,\n                            token_idx: TokenIdx(\n                                62,\n                            ),\n                        },\n                    },\n                    Expr::InheritedSymbol {\n                        ident: `pt`,\n                        token_idx: TokenIdx(\n                            66,\n                        ),\n                        inherited_symbol_idx: 0,\n                        inherited_symbol_kind: InheritedSymbolKind::RegularParameter {\n                            ident: `pt`,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 3,\n                        dot_token_idx: TokenIdx(\n                            63,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `to`,\n                            token_idx: TokenIdx(\n                                64,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            65,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            4..5,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            67,\n                        ),\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ab`,\n                        token_idx: TokenIdx(\n                            69,\n                        ),\n                        current_symbol_idx: 0,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 0,\n                        },\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ap`,\n                        token_idx: TokenIdx(\n                            73,\n                        ),\n                        current_symbol_idx: 1,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 1,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 6,\n                        dot_token_idx: TokenIdx(\n                            70,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `dot`,\n                            token_idx: TokenIdx(\n                                71,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            72,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            7..8,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            74,\n                        ),\n                    },\n                    Expr::Literal(\n                        TokenIdx(\n                            76,\n                        ),\n                    ),\n                    Expr::Binary {\n                        lopd: 8,\n                        opr: Comparison(\n                            Less,\n                        ),\n                        opr_token_idx: TokenIdx(\n                            75,\n                        ),\n                        ropd: 9,\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ap`,\n                        token_idx: TokenIdx(\n                            78,\n                        ),\n                        current_symbol_idx: 1,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 1,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 11,\n                        dot_token_idx: TokenIdx(\n                            79,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `norm`,\n                            token_idx: TokenIdx(\n                                80,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            81,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            12..12,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            82,\n                        ),\n                    },\n                    Expr::SelfValue(\n                        TokenIdx(\n                            88,\n                        ),\n                    ),\n                    Expr::Field {\n                        owner: 13,\n                        dot_token_idx: TokenIdx(\n                            89,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `end`,\n                            token_idx: TokenIdx(\n                                90,\n                            ),\n                        },\n                    },\n                    Expr::InheritedSymbol {\n                        ident: `pt`,\n                        token_idx: TokenIdx(\n                            94,\n                        ),\n                        inherited_symbol_idx: 0,\n                        inherited_symbol_kind: InheritedSymbolKind::RegularParameter {\n                            ident: `pt`,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 14,\n                        dot_token_idx: TokenIdx(\n                            91,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `to`,\n                            token_idx: TokenIdx(\n                                92,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            93,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            15..16,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            95,\n                        ),\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ab`,\n                        token_idx: TokenIdx(\n                            97,\n                        ),\n                        current_symbol_idx: 0,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 0,\n                        },\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `bp`,\n                        token_idx: TokenIdx(\n                            101,\n                        ),\n                        current_symbol_idx: 2,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 2,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 17,\n                        dot_token_idx: TokenIdx(\n                            98,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `dot`,\n                            token_idx: TokenIdx(\n                                99,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            100,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            18..19,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            102,\n                        ),\n                    },\n                    Expr::Literal(\n                        TokenIdx(\n                            104,\n                        ),\n                    ),\n                    Expr::Binary {\n                        lopd: 19,\n                        opr: Comparison(\n                            Greater,\n                        ),\n                        opr_token_idx: TokenIdx(\n                            103,\n                        ),\n                        ropd: 20,\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `bp`,\n                        token_idx: TokenIdx(\n                            106,\n                        ),\n                        current_symbol_idx: 2,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 2,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 22,\n                        dot_token_idx: TokenIdx(\n                            107,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `norm`,\n                            token_idx: TokenIdx(\n                                108,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            109,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            23..23,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            110,\n                        ),\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ab`,\n                        token_idx: TokenIdx(\n                            113,\n                        ),\n                        current_symbol_idx: 0,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 0,\n                        },\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ap`,\n                        token_idx: TokenIdx(\n                            117,\n                        ),\n                        current_symbol_idx: 1,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 1,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 24,\n                        dot_token_idx: TokenIdx(\n                            114,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `cross`,\n                            token_idx: TokenIdx(\n                                115,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            116,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            25..26,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            118,\n                        ),\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ab`,\n                        token_idx: TokenIdx(\n                            124,\n                        ),\n                        current_symbol_idx: 0,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 0,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 26,\n                        dot_token_idx: TokenIdx(\n                            119,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `abs`,\n                            token_idx: TokenIdx(\n                                120,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            121,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            27..27,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            122,\n                        ),\n                    },\n                    Expr::MethodCall {\n                        self_argument: 27,\n                        dot_token_idx: TokenIdx(\n                            125,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `norm`,\n                            token_idx: TokenIdx(\n                                126,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            127,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            28..28,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            128,\n                        ),\n                    },\n                    Expr::Binary {\n                        lopd: 28,\n                        opr: Closed(\n                            Div,\n                        ),\n                        opr_token_idx: TokenIdx(\n                            123,\n                        ),\n                        ropd: 29,\n                    },\n                    Expr::Block {\n                        stmts: ArenaIdxRange(\n                            5..8,\n                        ),\n                    },\n                ],\n            },\n            entity_path_expr_arena: Arena {\n                data: [],\n            },\n            stmt_arena: Arena {\n                data: [\n                    Stmt::Eval {\n                        expr_idx: 12,\n                    },\n                    Stmt::Eval {\n                        expr_idx: 23,\n                    },\n                    Stmt::Eval {\n                        expr_idx: 30,\n                    },\n                    Stmt::Let {\n                        let_token: LetToken {\n                            token_idx: TokenIdx(\n                                85,\n                            ),\n                        },\n                        let_variable_pattern: Ok(\n                            LetVariablesPattern {\n                                pattern: 2,\n                                variables: ArenaIdxRange(\n                                    2..3,\n                                ),\n                                colon_token: Ok(\n                                    None,\n                                ),\n                                ty: None,\n                            },\n                        ),\n                        assign_token: Ok(\n                            AssignToken(\n                                TokenIdx(\n                                    87,\n                                ),\n                            ),\n                        ),\n                        initial_value: Ok(\n                            16,\n                        ),\n                    },\n                    Stmt::IfElse {\n                        if_branch: IfBranch {\n                            if_token: IfToken {\n                                token_idx: TokenIdx(\n                                    96,\n                                ),\n                            },\n                            condition: Ok(\n                                21,\n                            ),\n                            eol_colon: Ok(\n                                EolColonToken(\n                                    TokenIdx(\n                                        105,\n                                    ),\n                                ),\n                            ),\n                            block: Ok(\n                                ArenaIdxRange(\n                                    1..2,\n                                ),\n                            ),\n                        },\n                        elif_branches: [],\n                        else_branch: Some(\n                            ElseBranch {\n                                else_token: ElseToken {\n                                    token_idx: TokenIdx(\n                                        111,\n                                    ),\n                                },\n                                eol_colon: Ok(\n                                    EolColonToken(\n                                        TokenIdx(\n                                            112,\n                                        ),\n                                    ),\n                                ),\n                                block: Ok(\n                                    ArenaIdxRange(\n                                        2..3,\n                                    ),\n                                ),\n                            },\n                        ),\n                    },\n                    Stmt::Let {\n                        let_token: LetToken {\n                            token_idx: TokenIdx(\n                                49,\n                            ),\n                        },\n                        let_variable_pattern: Ok(\n                            LetVariablesPattern {\n                                pattern: 0,\n                                variables: ArenaIdxRange(\n                                    0..1,\n                                ),\n                                colon_token: Ok(\n                                    None,\n                                ),\n                                ty: None,\n                            },\n                        ),\n                        assign_token: Ok(\n                            AssignToken(\n                                TokenIdx(\n                                    51,\n                                ),\n                            ),\n                        ),\n                        initial_value: Ok(\n                            1,\n                        ),\n                    },\n                    Stmt::Let {\n                        let_token: LetToken {\n                            token_idx: TokenIdx(\n                                57,\n                            ),\n                        },\n                        let_variable_pattern: Ok(\n                            LetVariablesPattern {\n                                pattern: 1,\n                                variables: ArenaIdxRange(\n                                    1..2,\n                                ),\n                                colon_token: Ok(\n                                    None,\n                                ),\n                                ty: None,\n                            },\n                        ),\n                        assign_token: Ok(\n                            AssignToken(\n                                TokenIdx(\n                                    59,\n                                ),\n                            ),\n                        ),\n                        initial_value: Ok(\n                            5,\n                        ),\n                    },\n                    Stmt::IfElse {\n                        if_branch: IfBranch {\n                            if_token: IfToken {\n                                token_idx: TokenIdx(\n                                    68,\n                                ),\n                            },\n                            condition: Ok(\n                                10,\n                            ),\n                            eol_colon: Ok(\n                                EolColonToken(\n                                    TokenIdx(\n                                        77,\n                                    ),\n                                ),\n                            ),\n                            block: Ok(\n                                ArenaIdxRange(\n                                    0..1,\n                                ),\n                            ),\n                        },\n                        elif_branches: [],\n                        else_branch: Some(\n                            ElseBranch {\n                                else_token: ElseToken {\n                                    token_idx: TokenIdx(\n                                        83,\n                                    ),\n                                },\n                                eol_colon: Ok(\n                                    EolColonToken(\n                                        TokenIdx(\n                                            84,\n                                        ),\n                                    ),\n                                ),\n                                block: Ok(\n                                    ArenaIdxRange(\n                                        3..5,\n                                    ),\n                                ),\n                            },\n                        ),\n                    },\n                ],\n            },\n            pattern_expr_region: PatternExprRegion {\n                pattern_expr_arena: Arena {\n                    data: [\n                        PatternExpr::Ident {\n                            ident_token: IdentToken {\n                                ident: `ab`,\n                                token_idx: TokenIdx(\n                                    50,\n                                ),\n                            },\n                            liason: None,\n                        },\n                        PatternExpr::Ident {\n                            ident_token: IdentToken {\n                                ident: `ap`,\n                                token_idx: TokenIdx(\n                                    58,\n                                ),\n                            },\n                            liason: None,\n                        },\n                        PatternExpr::Ident {\n                            ident_token: IdentToken {\n                                ident: `bp`,\n                                token_idx: TokenIdx(\n                                    86,\n                                ),\n                            },\n                            liason: None,\n                        },\n                    ],\n                },\n                pattern_infos: [\n                    Let,\n                    Let,\n                    Let,\n                ],\n                pattern_symbol_maps: [\n                    [\n                        (\n                            `ab`,\n                            0,\n                        ),\n                    ],\n                    [\n                        (\n                            `ap`,\n                            1,\n                        ),\n                    ],\n                    [\n                        (\n                            `bp`,\n                            2,\n                        ),\n                    ],\n                ],\n                pattern_symbol_arena: Arena {\n                    data: [\n                        PatternSymbol::Atom(\n                            0,\n                        ),\n                        PatternSymbol::Atom(\n                            1,\n                        ),\n                        PatternSymbol::Atom(\n                            2,\n                        ),\n                    ],\n                },\n            },\n            symbol_region: SymbolRegion {\n                inherited_symbol_arena: Arena {\n                    data: [\n                        InheritedSymbol {\n                            parent_symbol_idx: Current(\n                                0,\n                            ),\n                            kind: InheritedSymbolKind::RegularParameter {\n                                ident: `pt`,\n                            },\n                        },\n                    ],\n                },\n                current_symbol_arena: Arena {\n                    data: [\n                        CurrentSymbol {\n                            access_start: TokenIdx(\n                                51,\n                            ),\n                            access_end: Some(\n                                TokenIdxRangeEnd(\n                                    TokenIdx(\n                                        129,\n                                    ),\n                                ),\n                            ),\n                            variant: CurrentSymbolVariant::LetVariable {\n                                ident: `ab`,\n                                pattern_symbol_idx: 0,\n                            },\n                        },\n                        CurrentSymbol {\n                            access_start: TokenIdx(\n                                59,\n                            ),\n                            access_end: Some(\n                                TokenIdxRangeEnd(\n                                    TokenIdx(\n                                        129,\n                                    ),\n                                ),\n                            ),\n                            variant: CurrentSymbolVariant::LetVariable {\n                                ident: `ap`,\n                                pattern_symbol_idx: 1,\n                            },\n                        },\n                        CurrentSymbol {\n                            access_start: TokenIdx(\n                                87,\n                            ),\n                            access_end: Some(\n                                TokenIdxRangeEnd(\n                                    TokenIdx(\n                                        129,\n                                    ),\n                                ),\n                            ),\n                            variant: CurrentSymbolVariant::LetVariable {\n                                ident: `bp`,\n                                pattern_symbol_idx: 2,\n                            },\n                        },\n                    ],\n                },\n                allow_self_type: True,\n                allow_self_value: True,\n                pattern_ty_constraints: [],\n            },\n            roots: [\n                ExprRoot {\n                    kind: BlockExpr,\n                    expr: 31,\n                },\n            ],\n        },\n    },\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        0,\n    ),\n    kind: InheritedSymbolKind::RegularParameter {\n        ident: `pt`,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 31,
                                },
                                end: Position {
                                    line: 13,
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
                72,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 72;\n\ntoken = Token::Punctuation(\n    Punctuation::Bra(\n        Par,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 14,
                                    character: 17,
                                },
                                end: Position {
                                    line: 14,
                                    character: 18,
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
                                value: "\ntoken_idx = 78;\n\ntoken = Token::Ident(\n    `ap`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 1,\n    },\n    expr_region: ExprRegion {\n        data: ExprRegionData {\n            parent: Some(\n                ExprRegion {\n                    data: ExprRegionData {\n                        parent: Some(\n                            ExprRegion {\n                                data: ExprRegionData {\n                                    parent: None,\n                                    path: RegionPath::Decl(\n                                        DeclRegionPath::Impl(\n                                            ImplBlockId::Type(\n                                                TypeImplBlockId {\n                                                    module: `mnist_classifier::line_segment_sketch::line_segment`,\n                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                                    disambiguator: 0,\n                                                },\n                                            ),\n                                        ),\n                                    ),\n                                    expr_arena: Arena {\n                                        data: [\n                                            Expr::EntityPath {\n                                                entity_path_expr: 0,\n                                                path: Some(\n                                                    EntityPath::ModuleItem(\n                                                        ModuleItemPath::Type(\n                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                                        ),\n                                                    ),\n                                                ),\n                                            },\n                                        ],\n                                    },\n                                    entity_path_expr_arena: Arena {\n                                        data: [\n                                            EntityPathExpr::Root {\n                                                token_idx: TokenIdx(\n                                                    20,\n                                                ),\n                                                ident: `LineSegment`,\n                                                entity_path: EntityPath::ModuleItem(\n                                                    ModuleItemPath::Type(\n                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                                    ),\n                                                ),\n                                            },\n                                        ],\n                                    },\n                                    stmt_arena: Arena {\n                                        data: [],\n                                    },\n                                    pattern_expr_region: PatternExprRegion {\n                                        pattern_expr_arena: Arena {\n                                            data: [],\n                                        },\n                                        pattern_infos: [],\n                                        pattern_symbol_maps: [],\n                                        pattern_symbol_arena: Arena {\n                                            data: [],\n                                        },\n                                    },\n                                    symbol_region: SymbolRegion {\n                                        inherited_symbol_arena: Arena {\n                                            data: [],\n                                        },\n                                        current_symbol_arena: Arena {\n                                            data: [],\n                                        },\n                                        allow_self_type: True,\n                                        allow_self_value: False,\n                                        pattern_ty_constraints: [],\n                                    },\n                                    roots: [\n                                        ExprRoot {\n                                            kind: SelfType,\n                                            expr: 0,\n                                        },\n                                    ],\n                                },\n                            },\n                        ),\n                        path: RegionPath::Decl(\n                            DeclRegionPath::AssociatedItem(\n                                AssociatedItemId {\n                                    impl_block_id: ImplBlockId::Type(\n                                        TypeImplBlockId {\n                                            module: `mnist_classifier::line_segment_sketch::line_segment`,\n                                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                            disambiguator: 0,\n                                        },\n                                    ),\n                                    ident: `dist_to_point`,\n                                },\n                            ),\n                        ),\n                        expr_arena: Arena {\n                            data: [\n                                Expr::EntityPath {\n                                    entity_path_expr: 0,\n                                    path: Some(\n                                        EntityPath::ModuleItem(\n                                            ModuleItemPath::Type(\n                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                            ),\n                                        ),\n                                    ),\n                                },\n                                Expr::EntityPath {\n                                    entity_path_expr: 1,\n                                    path: Some(\n                                        EntityPath::ModuleItem(\n                                            ModuleItemPath::Type(\n                                                TypePath(`core::num::f32`, `Extern`),\n                                            ),\n                                        ),\n                                    ),\n                                },\n                            ],\n                        },\n                        entity_path_expr_arena: Arena {\n                            data: [\n                                EntityPathExpr::Root {\n                                    token_idx: TokenIdx(\n                                        44,\n                                    ),\n                                    ident: `Point2d`,\n                                    entity_path: EntityPath::ModuleItem(\n                                        ModuleItemPath::Type(\n                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),\n                                        ),\n                                    ),\n                                },\n                                EntityPathExpr::Root {\n                                    token_idx: TokenIdx(\n                                        47,\n                                    ),\n                                    ident: `f32`,\n                                    entity_path: EntityPath::ModuleItem(\n                                        ModuleItemPath::Type(\n                                            TypePath(`core::num::f32`, `Extern`),\n                                        ),\n                                    ),\n                                },\n                            ],\n                        },\n                        stmt_arena: Arena {\n                            data: [],\n                        },\n                        pattern_expr_region: PatternExprRegion {\n                            pattern_expr_arena: Arena {\n                                data: [\n                                    PatternExpr::Ident {\n                                        ident_token: IdentToken {\n                                            ident: `pt`,\n                                            token_idx: TokenIdx(\n                                                42,\n                                            ),\n                                        },\n                                        liason: None,\n                                    },\n                                ],\n                            },\n                            pattern_infos: [\n                                Parameter,\n                            ],\n                            pattern_symbol_maps: [\n                                [\n                                    (\n                                        `pt`,\n                                        0,\n                                    ),\n                                ],\n                            ],\n                            pattern_symbol_arena: Arena {\n                                data: [\n                                    PatternSymbol::Atom(\n                                        0,\n                                    ),\n                                ],\n                            },\n                        },\n                        symbol_region: SymbolRegion {\n                            inherited_symbol_arena: Arena {\n                                data: [],\n                            },\n                            current_symbol_arena: Arena {\n                                data: [\n                                    CurrentSymbol {\n                                        access_start: TokenIdx(\n                                            43,\n                                        ),\n                                        access_end: None,\n                                        variant: CurrentSymbolVariant::RegularParameter {\n                                            ident: `pt`,\n                                            pattern_symbol_idx: 0,\n                                        },\n                                    },\n                                ],\n                            },\n                            allow_self_type: True,\n                            allow_self_value: True,\n                            pattern_ty_constraints: [\n                                RegularParameter {\n                                    pattern: 0,\n                                    ty: 0,\n                                },\n                            ],\n                        },\n                        roots: [\n                            ExprRoot {\n                                kind: ReturnType,\n                                expr: 1,\n                            },\n                        ],\n                    },\n                },\n            ),\n            path: RegionPath::Defn(\n                DefnRegionPath::AssociatedItem(\n                    AssociatedItemId {\n                        impl_block_id: ImplBlockId::Type(\n                            TypeImplBlockId {\n                                module: `mnist_classifier::line_segment_sketch::line_segment`,\n                                ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),\n                                disambiguator: 0,\n                            },\n                        ),\n                        ident: `dist_to_point`,\n                    },\n                ),\n            ),\n            expr_arena: Arena {\n                data: [\n                    Expr::SelfValue(\n                        TokenIdx(\n                            52,\n                        ),\n                    ),\n                    Expr::MethodCall {\n                        self_argument: 0,\n                        dot_token_idx: TokenIdx(\n                            53,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `displacement`,\n                            token_idx: TokenIdx(\n                                54,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            55,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            1..1,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            56,\n                        ),\n                    },\n                    Expr::SelfValue(\n                        TokenIdx(\n                            60,\n                        ),\n                    ),\n                    Expr::Field {\n                        owner: 2,\n                        dot_token_idx: TokenIdx(\n                            61,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `start`,\n                            token_idx: TokenIdx(\n                                62,\n                            ),\n                        },\n                    },\n                    Expr::InheritedSymbol {\n                        ident: `pt`,\n                        token_idx: TokenIdx(\n                            66,\n                        ),\n                        inherited_symbol_idx: 0,\n                        inherited_symbol_kind: InheritedSymbolKind::RegularParameter {\n                            ident: `pt`,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 3,\n                        dot_token_idx: TokenIdx(\n                            63,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `to`,\n                            token_idx: TokenIdx(\n                                64,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            65,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            4..5,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            67,\n                        ),\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ab`,\n                        token_idx: TokenIdx(\n                            69,\n                        ),\n                        current_symbol_idx: 0,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 0,\n                        },\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ap`,\n                        token_idx: TokenIdx(\n                            73,\n                        ),\n                        current_symbol_idx: 1,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 1,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 6,\n                        dot_token_idx: TokenIdx(\n                            70,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `dot`,\n                            token_idx: TokenIdx(\n                                71,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            72,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            7..8,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            74,\n                        ),\n                    },\n                    Expr::Literal(\n                        TokenIdx(\n                            76,\n                        ),\n                    ),\n                    Expr::Binary {\n                        lopd: 8,\n                        opr: Comparison(\n                            Less,\n                        ),\n                        opr_token_idx: TokenIdx(\n                            75,\n                        ),\n                        ropd: 9,\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ap`,\n                        token_idx: TokenIdx(\n                            78,\n                        ),\n                        current_symbol_idx: 1,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 1,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 11,\n                        dot_token_idx: TokenIdx(\n                            79,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `norm`,\n                            token_idx: TokenIdx(\n                                80,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            81,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            12..12,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            82,\n                        ),\n                    },\n                    Expr::SelfValue(\n                        TokenIdx(\n                            88,\n                        ),\n                    ),\n                    Expr::Field {\n                        owner: 13,\n                        dot_token_idx: TokenIdx(\n                            89,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `end`,\n                            token_idx: TokenIdx(\n                                90,\n                            ),\n                        },\n                    },\n                    Expr::InheritedSymbol {\n                        ident: `pt`,\n                        token_idx: TokenIdx(\n                            94,\n                        ),\n                        inherited_symbol_idx: 0,\n                        inherited_symbol_kind: InheritedSymbolKind::RegularParameter {\n                            ident: `pt`,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 14,\n                        dot_token_idx: TokenIdx(\n                            91,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `to`,\n                            token_idx: TokenIdx(\n                                92,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            93,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            15..16,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            95,\n                        ),\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ab`,\n                        token_idx: TokenIdx(\n                            97,\n                        ),\n                        current_symbol_idx: 0,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 0,\n                        },\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `bp`,\n                        token_idx: TokenIdx(\n                            101,\n                        ),\n                        current_symbol_idx: 2,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 2,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 17,\n                        dot_token_idx: TokenIdx(\n                            98,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `dot`,\n                            token_idx: TokenIdx(\n                                99,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            100,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            18..19,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            102,\n                        ),\n                    },\n                    Expr::Literal(\n                        TokenIdx(\n                            104,\n                        ),\n                    ),\n                    Expr::Binary {\n                        lopd: 19,\n                        opr: Comparison(\n                            Greater,\n                        ),\n                        opr_token_idx: TokenIdx(\n                            103,\n                        ),\n                        ropd: 20,\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `bp`,\n                        token_idx: TokenIdx(\n                            106,\n                        ),\n                        current_symbol_idx: 2,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 2,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 22,\n                        dot_token_idx: TokenIdx(\n                            107,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `norm`,\n                            token_idx: TokenIdx(\n                                108,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            109,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            23..23,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            110,\n                        ),\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ab`,\n                        token_idx: TokenIdx(\n                            113,\n                        ),\n                        current_symbol_idx: 0,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 0,\n                        },\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ap`,\n                        token_idx: TokenIdx(\n                            117,\n                        ),\n                        current_symbol_idx: 1,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 1,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 24,\n                        dot_token_idx: TokenIdx(\n                            114,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `cross`,\n                            token_idx: TokenIdx(\n                                115,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            116,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            25..26,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            118,\n                        ),\n                    },\n                    Expr::CurrentSymbol {\n                        ident: `ab`,\n                        token_idx: TokenIdx(\n                            124,\n                        ),\n                        current_symbol_idx: 0,\n                        current_symbol_kind: CurrentSymbolKind::LetVariable {\n                            pattern_symbol_idx: 0,\n                        },\n                    },\n                    Expr::MethodCall {\n                        self_argument: 26,\n                        dot_token_idx: TokenIdx(\n                            119,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `abs`,\n                            token_idx: TokenIdx(\n                                120,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            121,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            27..27,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            122,\n                        ),\n                    },\n                    Expr::MethodCall {\n                        self_argument: 27,\n                        dot_token_idx: TokenIdx(\n                            125,\n                        ),\n                        ident_token: IdentToken {\n                            ident: `norm`,\n                            token_idx: TokenIdx(\n                                126,\n                            ),\n                        },\n                        implicit_arguments: None,\n                        lpar_token_idx: TokenIdx(\n                            127,\n                        ),\n                        nonself_arguments: ArenaIdxRange(\n                            28..28,\n                        ),\n                        rpar_token_idx: TokenIdx(\n                            128,\n                        ),\n                    },\n                    Expr::Binary {\n                        lopd: 28,\n                        opr: Closed(\n                            Div,\n                        ),\n                        opr_token_idx: TokenIdx(\n                            123,\n                        ),\n                        ropd: 29,\n                    },\n                    Expr::Block {\n                        stmts: ArenaIdxRange(\n                            5..8,\n                        ),\n                    },\n                ],\n            },\n            entity_path_expr_arena: Arena {\n                data: [],\n            },\n            stmt_arena: Arena {\n                data: [\n                    Stmt::Eval {\n                        expr_idx: 12,\n                    },\n                    Stmt::Eval {\n                        expr_idx: 23,\n                    },\n                    Stmt::Eval {\n                        expr_idx: 30,\n                    },\n                    Stmt::Let {\n                        let_token: LetToken {\n                            token_idx: TokenIdx(\n                                85,\n                            ),\n                        },\n                        let_variable_pattern: Ok(\n                            LetVariablesPattern {\n                                pattern: 2,\n                                variables: ArenaIdxRange(\n                                    2..3,\n                                ),\n                                colon_token: Ok(\n                                    None,\n                                ),\n                                ty: None,\n                            },\n                        ),\n                        assign_token: Ok(\n                            AssignToken(\n                                TokenIdx(\n                                    87,\n                                ),\n                            ),\n                        ),\n                        initial_value: Ok(\n                            16,\n                        ),\n                    },\n                    Stmt::IfElse {\n                        if_branch: IfBranch {\n                            if_token: IfToken {\n                                token_idx: TokenIdx(\n                                    96,\n                                ),\n                            },\n                            condition: Ok(\n                                21,\n                            ),\n                            eol_colon: Ok(\n                                EolColonToken(\n                                    TokenIdx(\n                                        105,\n                                    ),\n                                ),\n                            ),\n                            block: Ok(\n                                ArenaIdxRange(\n                                    1..2,\n                                ),\n                            ),\n                        },\n                        elif_branches: [],\n                        else_branch: Some(\n                            ElseBranch {\n                                else_token: ElseToken {\n                                    token_idx: TokenIdx(\n                                        111,\n                                    ),\n                                },\n                                eol_colon: Ok(\n                                    EolColonToken(\n                                        TokenIdx(\n                                            112,\n                                        ),\n                                    ),\n                                ),\n                                block: Ok(\n                                    ArenaIdxRange(\n                                        2..3,\n                                    ),\n                                ),\n                            },\n                        ),\n                    },\n                    Stmt::Let {\n                        let_token: LetToken {\n                            token_idx: TokenIdx(\n                                49,\n                            ),\n                        },\n                        let_variable_pattern: Ok(\n                            LetVariablesPattern {\n                                pattern: 0,\n                                variables: ArenaIdxRange(\n                                    0..1,\n                                ),\n                                colon_token: Ok(\n                                    None,\n                                ),\n                                ty: None,\n                            },\n                        ),\n                        assign_token: Ok(\n                            AssignToken(\n                                TokenIdx(\n                                    51,\n                                ),\n                            ),\n                        ),\n                        initial_value: Ok(\n                            1,\n                        ),\n                    },\n                    Stmt::Let {\n                        let_token: LetToken {\n                            token_idx: TokenIdx(\n                                57,\n                            ),\n                        },\n                        let_variable_pattern: Ok(\n                            LetVariablesPattern {\n                                pattern: 1,\n                                variables: ArenaIdxRange(\n                                    1..2,\n                                ),\n                                colon_token: Ok(\n                                    None,\n                                ),\n                                ty: None,\n                            },\n                        ),\n                        assign_token: Ok(\n                            AssignToken(\n                                TokenIdx(\n                                    59,\n                                ),\n                            ),\n                        ),\n                        initial_value: Ok(\n                            5,\n                        ),\n                    },\n                    Stmt::IfElse {\n                        if_branch: IfBranch {\n                            if_token: IfToken {\n                                token_idx: TokenIdx(\n                                    68,\n                                ),\n                            },\n                            condition: Ok(\n                                10,\n                            ),\n                            eol_colon: Ok(\n                                EolColonToken(\n                                    TokenIdx(\n                                        77,\n                                    ),\n                                ),\n                            ),\n                            block: Ok(\n                                ArenaIdxRange(\n                                    0..1,\n                                ),\n                            ),\n                        },\n                        elif_branches: [],\n                        else_branch: Some(\n                            ElseBranch {\n                                else_token: ElseToken {\n                                    token_idx: TokenIdx(\n                                        83,\n                                    ),\n                                },\n                                eol_colon: Ok(\n                                    EolColonToken(\n                                        TokenIdx(\n                                            84,\n                                        ),\n                                    ),\n                                ),\n                                block: Ok(\n                                    ArenaIdxRange(\n                                        3..5,\n                                    ),\n                                ),\n                            },\n                        ),\n                    },\n                ],\n            },\n            pattern_expr_region: PatternExprRegion {\n                pattern_expr_arena: Arena {\n                    data: [\n                        PatternExpr::Ident {\n                            ident_token: IdentToken {\n                                ident: `ab`,\n                                token_idx: TokenIdx(\n                                    50,\n                                ),\n                            },\n                            liason: None,\n                        },\n                        PatternExpr::Ident {\n                            ident_token: IdentToken {\n                                ident: `ap`,\n                                token_idx: TokenIdx(\n                                    58,\n                                ),\n                            },\n                            liason: None,\n                        },\n                        PatternExpr::Ident {\n                            ident_token: IdentToken {\n                                ident: `bp`,\n                                token_idx: TokenIdx(\n                                    86,\n                                ),\n                            },\n                            liason: None,\n                        },\n                    ],\n                },\n                pattern_infos: [\n                    Let,\n                    Let,\n                    Let,\n                ],\n                pattern_symbol_maps: [\n                    [\n                        (\n                            `ab`,\n                            0,\n                        ),\n                    ],\n                    [\n                        (\n                            `ap`,\n                            1,\n                        ),\n                    ],\n                    [\n                        (\n                            `bp`,\n                            2,\n                        ),\n                    ],\n                ],\n                pattern_symbol_arena: Arena {\n                    data: [\n                        PatternSymbol::Atom(\n                            0,\n                        ),\n                        PatternSymbol::Atom(\n                            1,\n                        ),\n                        PatternSymbol::Atom(\n                            2,\n                        ),\n                    ],\n                },\n            },\n            symbol_region: SymbolRegion {\n                inherited_symbol_arena: Arena {\n                    data: [\n                        InheritedSymbol {\n                            parent_symbol_idx: Current(\n                                0,\n                            ),\n                            kind: InheritedSymbolKind::RegularParameter {\n                                ident: `pt`,\n                            },\n                        },\n                    ],\n                },\n                current_symbol_arena: Arena {\n                    data: [\n                        CurrentSymbol {\n                            access_start: TokenIdx(\n                                51,\n                            ),\n                            access_end: Some(\n                                TokenIdxRangeEnd(\n                                    TokenIdx(\n                                        129,\n                                    ),\n                                ),\n                            ),\n                            variant: CurrentSymbolVariant::LetVariable {\n                                ident: `ab`,\n                                pattern_symbol_idx: 0,\n                            },\n                        },\n                        CurrentSymbol {\n                            access_start: TokenIdx(\n                                59,\n                            ),\n                            access_end: Some(\n                                TokenIdxRangeEnd(\n                                    TokenIdx(\n                                        129,\n                                    ),\n                                ),\n                            ),\n                            variant: CurrentSymbolVariant::LetVariable {\n                                ident: `ap`,\n                                pattern_symbol_idx: 1,\n                            },\n                        },\n                        CurrentSymbol {\n                            access_start: TokenIdx(\n                                87,\n                            ),\n                            access_end: Some(\n                                TokenIdxRangeEnd(\n                                    TokenIdx(\n                                        129,\n                                    ),\n                                ),\n                            ),\n                            variant: CurrentSymbolVariant::LetVariable {\n                                ident: `bp`,\n                                pattern_symbol_idx: 2,\n                            },\n                        },\n                    ],\n                },\n                allow_self_type: True,\n                allow_self_value: True,\n                pattern_ty_constraints: [],\n            },\n            roots: [\n                ExprRoot {\n                    kind: BlockExpr,\n                    expr: 31,\n                },\n            ],\n        },\n    },\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        59,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                129,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `ap`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 15,
                                    character: 12,
                                },
                                end: Position {
                                    line: 15,
                                    character: 14,
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
                84,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 84;\n\ntoken = Token::Punctuation(\n    Punctuation::Colon,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 16,
                                    character: 12,
                                },
                                end: Position {
                                    line: 16,
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
                90,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 90;\n\ntoken = Token::Ident(\n    `end`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 17,
                                    character: 26,
                                },
                                end: Position {
                                    line: 17,
                                    character: 29,
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
                96,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 96;\n\ntoken = Token::Keyword(\n    Keyword::Stmt(\n        If,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 18,
                                    character: 12,
                                },
                                end: Position {
                                    line: 18,
                                    character: 14,
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
                102,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 102;\n\ntoken = Token::Punctuation(\n    Punctuation::Ket(\n        Par,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 18,
                                    character: 24,
                                },
                                end: Position {
                                    line: 18,
                                    character: 25,
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
                108,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 108;\n\ntoken = Token::Ident(\n    `norm`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 19,
                                    character: 19,
                                },
                                end: Position {
                                    line: 19,
                                    character: 23,
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
                114,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 114;\n\ntoken = Token::Punctuation(\n    Punctuation::Dot,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 21,
                                    character: 18,
                                },
                                end: Position {
                                    line: 21,
                                    character: 19,
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
                120,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 120;\n\ntoken = Token::Ident(\n    `abs`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 21,
                                    character: 29,
                                },
                                end: Position {
                                    line: 21,
                                    character: 32,
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
                126,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 126;\n\ntoken = Token::Ident(\n    `norm`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 21,
                                    character: 38,
                                },
                                end: Position {
                                    line: 21,
                                    character: 42,
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