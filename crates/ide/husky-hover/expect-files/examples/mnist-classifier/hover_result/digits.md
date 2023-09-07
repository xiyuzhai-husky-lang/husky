Ok(
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
                                value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Keyword(\n    Keyword::Mod,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                5,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 4;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Keyword(\n    Keyword::Mod,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                9,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 8;\n\ntoken_line_group_idx = 4\n\ntoken = Token::Keyword(\n    Keyword::Mod,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                13,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 12;\n\ntoken_line_group_idx = 6\n\ntoken = Token::Keyword(\n    Keyword::Mod,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                17,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 16;\n\ntoken_line_group_idx = 8\n\ntoken = Token::Keyword(\n    Keyword::Mod,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 0,
                                },
                                end: Position {
                                    line: 8,
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
                21,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 20;\n\ntoken_line_group_idx = 10\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 0,
                                },
                                end: Position {
                                    line: 11,
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
                                value: "\ntoken_idx = 24;\n\ntoken_line_group_idx = 10\n\ntoken = Token::Ident(\n    `is_one`,\n);\n\ntoken_info = TokenInfo::UseExpr {\n    use_expr_idx: 0,\n    rule_idx: OnceUseRuleIdx(\n        10,\n    ),\n    state: OnceUseRuleState::Resolved {\n        original_symbol: Some(\n            EntitySymbol::MajorItem {\n                module_item_path: MajorItemPath::Fugitive(\n                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),\n                ),\n                node: MajorItemSynNode {\n                    syn_node_path: MajorItemSynNodePath::Fugitive(\n                        FugitiveSynNodePath {\n                            maybe_ambiguous_path: MaybeAmbiguousPath {\n                                path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),\n                                disambiguator: 0,\n                            },\n                        },\n                    ),\n                    visibility: Scope::Pub,\n                    ast_idx: 63,\n                    ident_token: IdentToken {\n                        ident: `is_one`,\n                        token_idx: TokenIdx(\n                            24,\n                        ),\n                    },\n                    block: Fugitive {\n                        path: FugitivePath(\n                            Id {\n                                value: 30,\n                            },\n                        ),\n                        body: Some(\n                            FugitiveBody {\n                                ast_idx_range: ArenaIdxRange(\n                                    48..51,\n                                ),\n                            },\n                        ),\n                    },\n                },\n            },\n        ),\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 13,
                                },
                                end: Position {
                                    line: 11,
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
                29,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 28;\n\ntoken_line_group_idx = 11\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 12,
                                },
                                end: Position {
                                    line: 12,
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
                33,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 32;\n\ntoken_line_group_idx = 12\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 0,
                                },
                                end: Position {
                                    line: 13,
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
                37,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 36;\n\ntoken_line_group_idx = 12\n\ntoken = Token::Ident(\n    `zero`,\n);\n\ntoken_info = TokenInfo::UseExpr {\n    use_expr_idx: 6,\n    rule_idx: OnceUseRuleIdx(\n        12,\n    ),\n    state: OnceUseRuleState::Resolved {\n        original_symbol: Some(\n            EntitySymbol::Submodule {\n                submodule_path: SubmodulePath(\n                    `mnist_classifier::digits::zero`,\n                ),\n                node: SubmoduleSynNode {\n                    syn_node_path: SubmoduleSynNodePath {\n                        maybe_ambiguous_path: MaybeAmbiguousPath {\n                            path: SubmodulePath(\n                                `mnist_classifier::digits::zero`,\n                            ),\n                            disambiguator: 0,\n                        },\n                    },\n                    visibility: Scope::PubUnder(\n                        `mnist_classifier::digits`,\n                    ),\n                    ast_idx: 0,\n                    ident_token: IdentToken {\n                        ident: `zero`,\n                        token_idx: TokenIdx(\n                            2,\n                        ),\n                    },\n                },\n            },\n        ),\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 14,
                                },
                                end: Position {
                                    line: 13,
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
                41,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 40;\n\ntoken_line_group_idx = 13\n\ntoken = Token::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 14,
                                    character: 4,
                                },
                                end: Position {
                                    line: 14,
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
                45,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 44;\n\ntoken_line_group_idx = 13\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                49,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 48;\n\ntoken_line_group_idx = 14\n\ntoken = Token::Keyword(\n    Keyword::Pronoun(\n        SelfValue,\n    ),\n);\n\ntoken_info = TokenInfo::UseExpr {\n    use_expr_idx: 13,\n    rule_idx: OnceUseRuleIdx(\n        4,\n    ),\n    state: OnceUseRuleState::Resolved {\n        original_symbol: Some(\n            EntitySymbol::SelfModule {\n                module_path: `mnist_classifier::digits`,\n            },\n        ),\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 15,
                                    character: 8,
                                },
                                end: Position {
                                    line: 15,
                                    character: 12,
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
                53,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 52;\n\ntoken_line_group_idx = 14\n\ntoken = Token::Ident(\n    `is_three`,\n);\n\ntoken_info = TokenInfo::UseExpr {\n    use_expr_idx: 11,\n    rule_idx: OnceUseRuleIdx(\n        22,\n    ),\n    state: OnceUseRuleState::Resolved {\n        original_symbol: Some(\n            EntitySymbol::MajorItem {\n                module_item_path: MajorItemPath::Fugitive(\n                    FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),\n                ),\n                node: MajorItemSynNode {\n                    syn_node_path: MajorItemSynNodePath::Fugitive(\n                        FugitiveSynNodePath {\n                            maybe_ambiguous_path: MaybeAmbiguousPath {\n                                path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),\n                                disambiguator: 0,\n                            },\n                        },\n                    ),\n                    visibility: Scope::PubUnder(\n                        `mnist_classifier::digits`,\n                    ),\n                    ast_idx: 29,\n                    ident_token: IdentToken {\n                        ident: `is_three`,\n                        token_idx: TokenIdx(\n                            27,\n                        ),\n                    },\n                    block: Fugitive {\n                        path: FugitivePath(\n                            Id {\n                                value: 40,\n                            },\n                        ),\n                        body: Some(\n                            FugitiveBody {\n                                ast_idx_range: ArenaIdxRange(\n                                    1..18,\n                                ),\n                            },\n                        ),\n                    },\n                },\n            },\n        ),\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 15,
                                    character: 21,
                                },
                                end: Position {
                                    line: 15,
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
                57,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 56;\n\ntoken_line_group_idx = 15\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                61,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 60;\n\ntoken_line_group_idx = 16\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 17,
                                    character: 0,
                                },
                                end: Position {
                                    line: 17,
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
                65,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 64;\n\ntoken_line_group_idx = 16\n\ntoken = Token::Ident(\n    `seven`,\n);\n\ntoken_info = TokenInfo::UseExpr {\n    use_expr_idx: 18,\n    rule_idx: OnceUseRuleIdx(\n        16,\n    ),\n    state: OnceUseRuleState::Resolved {\n        original_symbol: Some(\n            EntitySymbol::Submodule {\n                submodule_path: SubmodulePath(\n                    `mnist_classifier::digits::seven`,\n                ),\n                node: SubmoduleSynNode {\n                    syn_node_path: SubmoduleSynNodePath {\n                        maybe_ambiguous_path: MaybeAmbiguousPath {\n                            path: SubmodulePath(\n                                `mnist_classifier::digits::seven`,\n                            ),\n                            disambiguator: 0,\n                        },\n                    },\n                    visibility: Scope::PubUnder(\n                        `mnist_classifier::digits`,\n                    ),\n                    ast_idx: 6,\n                    ident_token: IdentToken {\n                        ident: `seven`,\n                        token_idx: TokenIdx(\n                            14,\n                        ),\n                    },\n                },\n            },\n        ),\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 17,
                                    character: 14,
                                },
                                end: Position {
                                    line: 17,
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
                69,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 68;\n\ntoken_line_group_idx = 17\n\ntoken = Token::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 18,
                                    character: 4,
                                },
                                end: Position {
                                    line: 18,
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
                73,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 72;\n\ntoken_line_group_idx = 17\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 18,
                                    character: 19,
                                },
                                end: Position {
                                    line: 18,
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
                77,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 76;\n\ntoken_line_group_idx = 18\n\ntoken = Token::Keyword(\n    Keyword::Pronoun(\n        SelfValue,\n    ),\n);\n\ntoken_info = TokenInfo::UseExpr {\n    use_expr_idx: 25,\n    rule_idx: OnceUseRuleIdx(\n        8,\n    ),\n    state: OnceUseRuleState::Resolved {\n        original_symbol: Some(\n            EntitySymbol::SelfModule {\n                module_path: `mnist_classifier::digits`,\n            },\n        ),\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 19,
                                    character: 8,
                                },
                                end: Position {
                                    line: 19,
                                    character: 12,
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
                81,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 80;\n\ntoken_line_group_idx = 18\n\ntoken = Token::Ident(\n    `is_nine`,\n);\n\ntoken_info = TokenInfo::UseExpr {\n    use_expr_idx: 23,\n    rule_idx: OnceUseRuleIdx(\n        26,\n    ),\n    state: OnceUseRuleState::Resolved {\n        original_symbol: Some(\n            EntitySymbol::MajorItem {\n                module_item_path: MajorItemPath::Fugitive(\n                    FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),\n                ),\n                node: MajorItemSynNode {\n                    syn_node_path: MajorItemSynNodePath::Fugitive(\n                        FugitiveSynNodePath {\n                            maybe_ambiguous_path: MaybeAmbiguousPath {\n                                path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),\n                                disambiguator: 0,\n                            },\n                        },\n                    ),\n                    visibility: Scope::PubUnder(\n                        `mnist_classifier::digits`,\n                    ),\n                    ast_idx: 38,\n                    ident_token: IdentToken {\n                        ident: `is_nine`,\n                        token_idx: TokenIdx(\n                            36,\n                        ),\n                    },\n                    block: Fugitive {\n                        path: FugitivePath(\n                            Id {\n                                value: 63,\n                            },\n                        ),\n                        body: Some(\n                            FugitiveBody {\n                                ast_idx_range: ArenaIdxRange(\n                                    17..28,\n                                ),\n                            },\n                        ),\n                    },\n                },\n            },\n        ),\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 19,
                                    character: 20,
                                },
                                end: Position {
                                    line: 19,
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
                85,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 84;\n\ntoken_line_group_idx = 19\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Star,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 21,
                                    character: 11,
                                },
                                end: Position {
                                    line: 21,
                                    character: 12,
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