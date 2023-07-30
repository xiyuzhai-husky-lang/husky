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
                2,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 2;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Ident(\n    `Prop`,\n);\n\ntoken_info = TokenInfo::EntityNode(\n    EntitySynNodePath::ModuleItem(\n        ModuleItemSynNodePath::Type(\n            TypeSynNodePath {\n                maybe_ambiguous_path: MaybeAmbiguousPath {\n                    path: TypePath(`core::logic::Prop`, `Extern`),\n                    disambiguator: 0,\n                },\n            },\n        ),\n    ),\n    ModuleItem {\n        module_item_kind: Type(\n            Extern,\n        ),\n        connection: Connected,\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 11,
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
                4,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 4;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                                value: "\ntoken_idx = 6;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `LogicAnd`,\n);\n\ntoken_info = TokenInfo::EntityNode(\n    EntitySynNodePath::ModuleItem(\n        ModuleItemSynNodePath::Type(\n            TypeSynNodePath {\n                maybe_ambiguous_path: MaybeAmbiguousPath {\n                    path: TypePath(`core::logic::LogicAnd`, `Structure`),\n                    disambiguator: 0,\n                },\n            },\n        ),\n    ),\n    ModuleItem {\n        module_item_kind: Type(\n            Structure,\n        ),\n        connection: Connected,\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 14,
                                },
                                end: Position {
                                    line: 2,
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
                8,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 8;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `P`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `P`,\n                token_idx: TokenIdx(\n                    8,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        9,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [],\n            attrs: TemplateParameterAttrs {\n                phantom: false,\n            },\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentToken {\n                ident: `P`,\n                token_idx: TokenIdx(\n                    8,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 23,
                                },
                                end: Position {
                                    line: 2,
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
                10,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 10;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `Prop`,\n);\n\ntoken_info = TokenInfo::Entity(\n    EntityPath::ModuleItem(\n        ModuleItemPath::Type(\n            TypePath(`core::logic::Prop`, `Extern`),\n        ),\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 26,
                                },
                                end: Position {
                                    line: 2,
                                    character: 30,
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
                                value: "\ntoken_idx = 12;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `Q`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `Q`,\n                token_idx: TokenIdx(\n                    12,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        13,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [],\n            attrs: TemplateParameterAttrs {\n                phantom: false,\n            },\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentToken {\n                ident: `Q`,\n                token_idx: TokenIdx(\n                    12,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 32,
                                },
                                end: Position {
                                    line: 2,
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
                14,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 14;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `Prop`,\n);\n\ntoken_info = TokenInfo::Entity(\n    EntityPath::ModuleItem(\n        ModuleItemPath::Type(\n            TypePath(`core::logic::Prop`, `Extern`),\n        ),\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 35,
                                },
                                end: Position {
                                    line: 2,
                                    character: 39,
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
                                value: "\ntoken_idx = 16;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Curl,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 41,
                                },
                                end: Position {
                                    line: 2,
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
                18,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 18;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                20,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 20;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                22,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 22;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 9,
                                },
                                end: Position {
                                    line: 4,
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
                24,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 24;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 12,
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
                26,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 26;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 0,
                                },
                                end: Position {
                                    line: 7,
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
                28,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 28;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Ident(\n    `LogicOr`,\n);\n\ntoken_info = TokenInfo::EntityNode(\n    EntitySynNodePath::ModuleItem(\n        ModuleItemSynNodePath::Type(\n            TypeSynNodePath {\n                maybe_ambiguous_path: MaybeAmbiguousPath {\n                    path: TypePath(`core::logic::LogicOr`, `Inductive`),\n                    disambiguator: 0,\n                },\n            },\n        ),\n    ),\n    ModuleItem {\n        module_item_kind: Type(\n            Inductive,\n        ),\n        connection: Connected,\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 14,
                                },
                                end: Position {
                                    line: 7,
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
                30,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 30;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Ident(\n    `P`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `P`,\n                token_idx: TokenIdx(\n                    30,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        31,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [],\n            attrs: TemplateParameterAttrs {\n                phantom: false,\n            },\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentToken {\n                ident: `P`,\n                token_idx: TokenIdx(\n                    30,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 22,
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
                32,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 32;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Ident(\n    `Prop`,\n);\n\ntoken_info = TokenInfo::Entity(\n    EntityPath::ModuleItem(\n        ModuleItemPath::Type(\n            TypePath(`core::logic::Prop`, `Extern`),\n        ),\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 25,
                                },
                                end: Position {
                                    line: 7,
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
                34,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 34;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Ident(\n    `Q`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `Q`,\n                token_idx: TokenIdx(\n                    34,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        35,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [],\n            attrs: TemplateParameterAttrs {\n                phantom: false,\n            },\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentToken {\n                ident: `Q`,\n                token_idx: TokenIdx(\n                    34,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 31,
                                },
                                end: Position {
                                    line: 7,
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
                36,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 36;\n\ntoken_line_group_idx = 2\n\ntoken = Token::Ident(\n    `Prop`,\n);\n\ntoken_info = TokenInfo::Entity(\n    EntityPath::ModuleItem(\n        ModuleItemPath::Type(\n            TypePath(`core::logic::Prop`, `Extern`),\n        ),\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 34,
                                },
                                end: Position {
                                    line: 7,
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
                38,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 38;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Vertical,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                40,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 40;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 6,
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
                42,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 42;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 8,
                                },
                                end: Position {
                                    line: 8,
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
                44,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 44;\n\ntoken_line_group_idx = 4\n\ntoken = Token::Ident(\n    `Right`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 2,
                                },
                                end: Position {
                                    line: 9,
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
                46,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 46;\n\ntoken_line_group_idx = 4\n\ntoken = Token::Ident(\n    `Q`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 8,
                                },
                                end: Position {
                                    line: 9,
                                    character: 9,
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