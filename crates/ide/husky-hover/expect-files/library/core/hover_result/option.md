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
                                value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                1,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 1;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 4,
                                },
                                end: Position {
                                    line: 0,
                                    character: 7,
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
                2,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 2;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Ident(\n    `Option`,\n);\n\ntoken_info = TokenInfo::UseExpr {\n    use_expr_idx: 1,\n    rule_idx: OnceUseRuleIdx(\n        0,\n    ),\n    state: OnceUseRuleState::Resolved {\n        original_symbol: Some(\n            EntitySymbol::ModuleItem {\n                module_item_path: ModuleItemPath::Type(\n                    TypePath(`core::option::Option`, `Enum`),\n                ),\n                node: ModuleItemNode {\n                    node_path: ModuleItemNodePath::Type(\n                        TypeNodePath {\n                            maybe_ambiguous_path: MaybeAmbiguousPath {\n                                path: TypePath(`core::option::Option`, `Enum`),\n                                disambiguator: 0,\n                            },\n                        },\n                    ),\n                    visibility: Scope::Pub,\n                    ast_idx: 3,\n                    ident_token: IdentToken {\n                        ident: `Option`,\n                        token_idx: TokenIdx(\n                            7,\n                        ),\n                    },\n                    block: Type {\n                        path: TypePath(\n                            Id {\n                                value: 29,\n                            },\n                        ),\n                        variants: Some(\n                            TypeVariants {\n                                ast_idx_range: ArenaIdxRange(\n                                    0..2,\n                                ),\n                            },\n                        ),\n                    },\n                },\n            },\n        ),\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 8,
                                },
                                end: Position {
                                    line: 0,
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
                3,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 3;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 14,
                                },
                                end: Position {
                                    line: 0,
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
                4,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 4;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Star,\n    ),\n);\n\ntoken_info = TokenInfo::UseExprStar;\n\nuse expr star\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 16,
                                },
                                end: Position {
                                    line: 0,
                                    character: 17,
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
                5,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 5;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                6,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 6;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Keyword(\n    Keyword::TypeEntity(\n        Enum,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 4,
                                },
                                end: Position {
                                    line: 2,
                                    character: 8,
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
                7,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 7;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `Option`,\n);\n\ntoken_info = TokenInfo::EntityNode(\n    EntityNodePath::ModuleItem(\n        ModuleItemNodePath::Type(\n            TypeNodePath {\n                maybe_ambiguous_path: MaybeAmbiguousPath {\n                    path: TypePath(`core::option::Option`, `Enum`),\n                    disambiguator: 0,\n                },\n            },\n        ),\n    ),\n    ModuleItem {\n        module_item_kind: Type(\n            Enum,\n        ),\n        connection: Connected,\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 9,
                                },
                                end: Position {
                                    line: 2,
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
                8,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 8;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::LaOrLt,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 15,
                                },
                                end: Position {
                                    line: 2,
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
                9,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 9;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `T`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `T`,\n                token_idx: TokenIdx(\n                    9,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        10,\n    ),\n    access_end: None,\n    variant: CurrentSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {\n            ident_token: IdentToken {\n                ident: `T`,\n                token_idx: TokenIdx(\n                    9,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 16,
                                },
                                end: Position {
                                    line: 2,
                                    character: 17,
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
                10,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 10;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 17,
                                },
                                end: Position {
                                    line: 2,
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
                11,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 11;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Vertical,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 0,
                                },
                                end: Position {
                                    line: 3,
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
                12,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 12;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Ident(\n    `Some`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 2,
                                },
                                end: Position {
                                    line: 3,
                                    character: 6,
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
                13,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 13;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 6,
                                },
                                end: Position {
                                    line: 3,
                                    character: 7,
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
                14,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 14;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Ident(\n    `T`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 7,
                                },
                                end: Position {
                                    line: 3,
                                    character: 8,
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
                15,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 15;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 8,
                                },
                                end: Position {
                                    line: 3,
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
                16,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 16;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Vertical,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 0,
                                },
                                end: Position {
                                    line: 4,
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
                17,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 17;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Ident(\n    `None`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 2,
                                },
                                end: Position {
                                    line: 4,
                                    character: 6,
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