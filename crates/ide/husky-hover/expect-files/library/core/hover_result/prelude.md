[
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
                            value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
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
            4,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 3;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 13,
                            },
                            end: Position {
                                line: 0,
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
            7,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 6;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Star,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 22,
                            },
                            end: Position {
                                line: 0,
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
            10,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 9;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Keyword(\n    Keyword::Pronoun(\n        Crate,\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::UseExpr(\n            6,\n        ),\n        data: TokenInfoData::UseExpr {\n            use_expr_idx: 6,\n            rule_idx: OnceUseRuleIdx(\n                1,\n            ),\n            state: OnceUseRuleState::Resolved {\n                original_symbol: Some(\n                    EntitySymbol::CrateRoot {\n                        root_module_path: `core`,\n                    },\n                ),\n            },\n        },\n    },\n);\n\nuse\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 8,
                            },
                            end: Position {
                                line: 1,
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
            13,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 12;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 18,
                            },
                            end: Position {
                                line: 1,
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
            16,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 15;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = None;\n\n\n",
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
            19,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 18;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `raw_bits`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::UseExpr(\n            8,\n        ),\n        data: TokenInfoData::UseExpr {\n            use_expr_idx: 8,\n            rule_idx: OnceUseRuleIdx(\n                13,\n            ),\n            state: OnceUseRuleState::Resolved {\n                original_symbol: Some(\n                    EntitySymbol::Submodule {\n                        submodule_item_path: SubmoduleItemPath(\n                            ItemPathId {\n                                data: ItemPathData::SubmoduleItem(\n                                    SubmoduleItemPathData {\n                                        submodule_path: SubmodulePath(\n                                            `core::raw_bits`,\n                                        ),\n                                    },\n                                ),\n                            },\n                        ),\n                    },\n                ),\n            },\n        },\n    },\n);\n\nuse\n",
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
            22,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 21;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
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
            25,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 24;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 3,
                                character: 13,
                            },
                            end: Position {
                                line: 3,
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
            28,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 27;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Star,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 3,
                                character: 20,
                            },
                            end: Position {
                                line: 3,
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
            31,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 30;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Keyword(\n    Keyword::Pronoun(\n        Crate,\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::UseExpr(\n            15,\n        ),\n        data: TokenInfoData::UseExpr {\n            use_expr_idx: 15,\n            rule_idx: OnceUseRuleIdx(\n                4,\n            ),\n            state: OnceUseRuleState::Resolved {\n                original_symbol: Some(\n                    EntitySymbol::CrateRoot {\n                        root_module_path: `core`,\n                    },\n                ),\n            },\n        },\n    },\n);\n\nuse\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 8,
                            },
                            end: Position {
                                line: 4,
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
            34,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 33;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 21,
                            },
                            end: Position {
                                line: 4,
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
            37,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 36;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = None;\n\n\n",
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
            40,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 39;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Ident(\n    `marker`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::UseExpr(\n            17,\n        ),\n        data: TokenInfoData::UseExpr {\n            use_expr_idx: 17,\n            rule_idx: OnceUseRuleIdx(\n                16,\n            ),\n            state: OnceUseRuleState::Resolved {\n                original_symbol: Some(\n                    EntitySymbol::Submodule {\n                        submodule_item_path: SubmoduleItemPath(\n                            ItemPathId {\n                                data: ItemPathData::SubmoduleItem(\n                                    SubmoduleItemPathData {\n                                        submodule_path: SubmodulePath(\n                                            `core::marker`,\n                                        ),\n                                    },\n                                ),\n                            },\n                        ),\n                    },\n                ),\n            },\n        },\n    },\n);\n\nuse\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 15,
                            },
                            end: Position {
                                line: 5,
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
            43,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 42;\n\ntoken_line_group_idx = 6\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 0,
                            },
                            end: Position {
                                line: 6,
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
            46,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 45;\n\ntoken_line_group_idx = 6\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 13,
                            },
                            end: Position {
                                line: 6,
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
            49,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 48;\n\ntoken_line_group_idx = 6\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Star,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 22,
                            },
                            end: Position {
                                line: 6,
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
            52,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 51;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Keyword(\n    Keyword::Pronoun(\n        Crate,\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::UseExpr(\n            24,\n        ),\n        data: TokenInfoData::UseExpr {\n            use_expr_idx: 24,\n            rule_idx: OnceUseRuleIdx(\n                7,\n            ),\n            state: OnceUseRuleState::Resolved {\n                original_symbol: Some(\n                    EntitySymbol::CrateRoot {\n                        root_module_path: `core`,\n                    },\n                ),\n            },\n        },\n    },\n);\n\nuse\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 7,
                                character: 8,
                            },
                            end: Position {
                                line: 7,
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
            55,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 54;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 7,
                                character: 21,
                            },
                            end: Position {
                                line: 7,
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
            58,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 57;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 8,
                                character: 4,
                            },
                            end: Position {
                                line: 8,
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
            61,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 60;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Ident(\n    `cmp`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::UseExpr(\n            26,\n        ),\n        data: TokenInfoData::UseExpr {\n            use_expr_idx: 26,\n            rule_idx: OnceUseRuleIdx(\n                19,\n            ),\n            state: OnceUseRuleState::Resolved {\n                original_symbol: Some(\n                    EntitySymbol::Submodule {\n                        submodule_item_path: SubmoduleItemPath(\n                            ItemPathId {\n                                data: ItemPathData::SubmoduleItem(\n                                    SubmoduleItemPathData {\n                                        submodule_path: SubmodulePath(\n                                            `core::cmp`,\n                                        ),\n                                    },\n                                ),\n                            },\n                        ),\n                    },\n                ),\n            },\n        },\n    },\n);\n\nuse\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 8,
                                character: 15,
                            },
                            end: Position {
                                line: 8,
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
            64,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 63;\n\ntoken_line_group_idx = 9\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 9,
                                character: 0,
                            },
                            end: Position {
                                line: 9,
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
            67,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 66;\n\ntoken_line_group_idx = 9\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 9,
                                character: 13,
                            },
                            end: Position {
                                line: 9,
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
            70,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 69;\n\ntoken_line_group_idx = 9\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Star,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 9,
                                character: 22,
                            },
                            end: Position {
                                line: 9,
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
            73,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 72;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Keyword(\n    Keyword::Pronoun(\n        Crate,\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::UseExpr(\n            33,\n        ),\n        data: TokenInfoData::UseExpr {\n            use_expr_idx: 33,\n            rule_idx: OnceUseRuleIdx(\n                10,\n            ),\n            state: OnceUseRuleState::Resolved {\n                original_symbol: Some(\n                    EntitySymbol::CrateRoot {\n                        root_module_path: `core`,\n                    },\n                ),\n            },\n        },\n    },\n);\n\nuse\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 8,
                            },
                            end: Position {
                                line: 10,
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
            76,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 75;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 18,
                            },
                            end: Position {
                                line: 10,
                                character: 20,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
]