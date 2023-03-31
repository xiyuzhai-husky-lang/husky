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
                                value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                15,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 15;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `normalize`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 1,
                                    character: 25,
                                },
                                end: Position {
                                    line: 1,
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
                30,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 30;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Punctuation(\n    Punctuation::ColonColon,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                45,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 45;\n\ntoken_line_group_idx = 5\n\ntoken = Token::Keyword(\n    Keyword::Pronoun(\n        Crate,\n    ),\n);\n\ntoken_info = TokenInfo::UseExpr {\n    use_expr_idx: 24,\n    rule_idx: UseExprRuleIdx(\n        5,\n    ),\n    state: UseExprRuleState::Resolved {\n        original_symbol: CrateRoot {\n            root_module: ModulePath(\n                Id {\n                    value: 40,\n                },\n            ),\n        },\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 5,
                                    character: 4,
                                },
                                end: Position {
                                    line: 5,
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
                60,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 60;\n\ntoken_line_group_idx = 7\n\ntoken = Token::Punctuation(\n    Punctuation::ColonColon,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 9,
                                },
                                end: Position {
                                    line: 7,
                                    character: 11,
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
                75,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 75;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Punctuation(\n    Punctuation::Ket(\n        Box,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 56,
                                },
                                end: Position {
                                    line: 10,
                                    character: 57,
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
                                value: "\ntoken_idx = 90;\n\ntoken_line_group_idx = 11\n\ntoken = Token::Ident(\n    `cc`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {\n        ident: `cc`,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        0,\n    ),\n    kind: InheritedSymbolKind::ExplicitParameter {\n        ident: `cc`,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 12,
                                },
                                end: Position {
                                    line: 13,
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
                105,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 105;\n\ntoken_line_group_idx = 13\n\ntoken = Token::Punctuation(\n    Punctuation::Bra(\n        Par,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 16,
                                    character: 3,
                                },
                                end: Position {
                                    line: 16,
                                    character: 4,
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
                                value: "\ntoken_idx = 120;\n\ntoken_line_group_idx = 15\n\ntoken = Token::Punctuation(\n    Punctuation::Dot,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 18,
                                    character: 32,
                                },
                                end: Position {
                                    line: 18,
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
                135,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 135;\n\ntoken_line_group_idx = 17\n\ntoken = Token::Ident(\n    `n`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 20,
                                    character: 16,
                                },
                                end: Position {
                                    line: 20,
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
                150,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 150;\n\ntoken_line_group_idx = 19\n\ntoken = Token::Ident(\n    `ilen`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 22,
                                    character: 37,
                                },
                                end: Position {
                                    line: 22,
                                    character: 41,
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
                165,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 165;\n\ntoken_line_group_idx = 20\n\ntoken = Token::Punctuation(\n    Punctuation::Bra(\n        Par,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 23,
                                    character: 50,
                                },
                                end: Position {
                                    line: 23,
                                    character: 51,
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
                180,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 180;\n\ntoken_line_group_idx = 23\n\ntoken = Token::Punctuation(\n    Punctuation::Eq,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 26,
                                    character: 24,
                                },
                                end: Position {
                                    line: 26,
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
                195,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 195;\n\ntoken_line_group_idx = 24\n\ntoken = Token::Punctuation(\n    Punctuation::Comma,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 29,
                                    character: 9,
                                },
                                end: Position {
                                    line: 29,
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
                210,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 210;\n\ntoken_line_group_idx = 25\n\ntoken = Token::Keyword(\n    Keyword::Stmt(\n        Require,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 34,
                                    character: 4,
                                },
                                end: Position {
                                    line: 34,
                                    character: 11,
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
                225,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 225;\n\ntoken_line_group_idx = 26\n\ntoken = Token::WordOpr(\n    WordOpr::Be,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 35,
                                    character: 59,
                                },
                                end: Position {
                                    line: 35,
                                    character: 61,
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
                240,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 240;\n\ntoken_line_group_idx = 28\n\ntoken = Token::Punctuation(\n    Punctuation::Eq,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 38,
                                    character: 19,
                                },
                                end: Position {
                                    line: 38,
                                    character: 20,
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
                255,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 255;\n\ntoken_line_group_idx = 29\n\ntoken = Token::Punctuation(\n    Punctuation::Dot,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 39,
                                    character: 35,
                                },
                                end: Position {
                                    line: 39,
                                    character: 36,
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
                270,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 270;\n\ntoken_line_group_idx = 30\n\ntoken = Token::Ident(\n    `major_line_segment_sketch`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Form(\n                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 40,
                                    character: 12,
                                },
                                end: Position {
                                    line: 40,
                                    character: 37,
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
                285,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 285;\n\ntoken_line_group_idx = 31\n\ntoken = Token::Keyword(\n    Keyword::Stmt(\n        Let,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 42,
                                    character: 4,
                                },
                                end: Position {
                                    line: 42,
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
                300,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 300;\n\ntoken_line_group_idx = 33\n\ntoken = Token::Ident(\n    `norm`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 44,
                                    character: 28,
                                },
                                end: Position {
                                    line: 44,
                                    character: 32,
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