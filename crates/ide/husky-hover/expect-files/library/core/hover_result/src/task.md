```rust
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
            2,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 1;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Keyword(\n    Keyword::Trait,\n);\n\ntoken_info = None;\n\n\n",
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
            3,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 2;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Ident(\n    `IsTask`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::MajorItem(\n                MajorItemSynNodePath::Trait(\n                    TraitSynNodePath(\n                        ItemSynNodePathId {\n                            data: ItemSynNodePathData::MajorItem(\n                                MajorItemSynNodePathData::Trait(\n                                    TraitSynNodePathData {\n                                        disambiguated_item_path: DisambiguatedItemPath {\n                                            maybe_ambiguous_item_path: TraitPath(`core::task::IsTask`),\n                                            disambiguator: 0,\n                                        },\n                                    },\n                                ),\n                            ),\n                        },\n                    ),\n                ),\n            ),\n            EntityKind::MajorItem {\n                module_item_kind: MajorItemKind::Trait,\n                connection: MajorItemConnectionKind::Connected,\n            },\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 10,
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
                            value: "\ntoken_idx = 3;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
                            value: "This is a paradigm\ntoken_idx = 4;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Keyword(\n    Keyword::Form(\n        Type,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 4,
                            },
                            end: Position {
                                line: 1,
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
            6,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 5;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Ident(\n    `Backend`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::AssocItem(\n                AssocItemSynNodePath::TraitItem(\n                    TraitItemSynNodePath(\n                        ItemSynNodePathId {\n                            data: ItemSynNodePathData::AssocItem(\n                                AssocItemSynNodePathData::TraitItem(\n                                    TraitItemSynNodePathData {\n                                        parent_trai_syn_node_path: TraitSynNodePath(\n                                            ItemSynNodePathId {\n                                                data: ItemSynNodePathData::MajorItem(\n                                                    MajorItemSynNodePathData::Trait(\n                                                        TraitSynNodePathData {\n                                                            disambiguated_item_path: DisambiguatedItemPath {\n                                                                maybe_ambiguous_item_path: TraitPath(`core::task::IsTask`),\n                                                                disambiguator: 0,\n                                                            },\n                                                        },\n                                                    ),\n                                                ),\n                                            },\n                                        ),\n                                        disambiguated_item_path: DisambiguatedItemPath {\n                                            maybe_ambiguous_item_path: TraitItemPath(\n                                                `core::task::IsTask::Backend`,\n                                                TraitItemKind::AssocType,\n                                            ),\n                                            disambiguator: 0,\n                                        },\n                                    },\n                                ),\n                            ),\n                        },\n                    ),\n                ),\n            ),\n            EntityKind::AssocItem {\n                assoc_item_kind: AssocItemKind::TraitItem(\n                    TraitItemKind::AssocType,\n                ),\n            },\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 9,
                            },
                            end: Position {
                                line: 1,
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
            7,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 6;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 16,
                            },
                            end: Position {
                                line: 1,
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
            8,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 7;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Ident(\n    `IsBackend`,\n);\n\ntoken_info = None;\n\n\n",
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
            9,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 8;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Semicolon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 27,
                            },
                            end: Position {
                                line: 1,
                                character: 28,
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
                            value: "This is a paradigm\ntoken_idx = 9;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Keyword(\n    Keyword::Form(\n        Type,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
            11,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 10;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `Frontend`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::AssocItem(\n                AssocItemSynNodePath::TraitItem(\n                    TraitItemSynNodePath(\n                        ItemSynNodePathId {\n                            data: ItemSynNodePathData::AssocItem(\n                                AssocItemSynNodePathData::TraitItem(\n                                    TraitItemSynNodePathData {\n                                        parent_trai_syn_node_path: TraitSynNodePath(\n                                            ItemSynNodePathId {\n                                                data: ItemSynNodePathData::MajorItem(\n                                                    MajorItemSynNodePathData::Trait(\n                                                        TraitSynNodePathData {\n                                                            disambiguated_item_path: DisambiguatedItemPath {\n                                                                maybe_ambiguous_item_path: TraitPath(`core::task::IsTask`),\n                                                                disambiguator: 0,\n                                                            },\n                                                        },\n                                                    ),\n                                                ),\n                                            },\n                                        ),\n                                        disambiguated_item_path: DisambiguatedItemPath {\n                                            maybe_ambiguous_item_path: TraitItemPath(\n                                                `core::task::IsTask::Frontend`,\n                                                TraitItemKind::AssocType,\n                                            ),\n                                            disambiguator: 0,\n                                        },\n                                    },\n                                ),\n                            ),\n                        },\n                    ),\n                ),\n            ),\n            EntityKind::AssocItem {\n                assoc_item_kind: AssocItemKind::TraitItem(\n                    TraitItemKind::AssocType,\n                ),\n            },\n        ),\n    },\n);\n\n\n",
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
            12,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 11;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
            13,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 12;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `IsFrontend`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 2,
                                character: 19,
                            },
                            end: Position {
                                line: 2,
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
            14,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 13;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Semicolon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 2,
                                character: 29,
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
            15,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 14;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
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
            16,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "This is a paradigm\ntoken_idx = 15;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Keyword(\n    Keyword::Form(\n        Type,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 4,
                            },
                            end: Position {
                                line: 4,
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
            17,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 16;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Keyword(\n    Keyword::Var,\n);\n\ntoken_info = None;\n\n\n",
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
            18,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 17;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Ident(\n    `Task`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::MajorItem(\n                MajorItemSynNodePath::Form(\n                    FormSynNodePath(`core::task::Task`, `TypeVar`, (0)),\n                ),\n            ),\n            EntityKind::MajorItem {\n                module_item_kind: MajorItemKind::Form(\n                    MajorFormKind::TypeVar,\n                ),\n                connection: MajorItemConnectionKind::Connected,\n            },\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 13,
                            },
                            end: Position {
                                line: 4,
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
            19,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 18;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
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
            20,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "This is a paradigm\ntoken_idx = 19;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Keyword(\n    Keyword::Form(\n        Static,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 4,
                            },
                            end: Position {
                                line: 6,
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
            21,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 20;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Keyword(\n    Keyword::Var,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 11,
                            },
                            end: Position {
                                line: 6,
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
            22,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 21;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Ident(\n    `TASK`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::MajorItem(\n                MajorItemSynNodePath::Form(\n                    FormSynNodePath(`core::task::TASK`, `StaticVar`, (0)),\n                ),\n            ),\n            EntityKind::MajorItem {\n                module_item_kind: MajorItemKind::Form(\n                    MajorFormKind::StaticVar,\n                ),\n                connection: MajorItemConnectionKind::Connected,\n            },\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 15,
                            },
                            end: Position {
                                line: 6,
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
            23,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 22;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 19,
                            },
                            end: Position {
                                line: 6,
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
            24,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 23;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Ident(\n    `Task`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            0,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Form(\n                    FormPath(`core::task::Task`, `TypeVar`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Form(\n                    FormPath(`core::task::Task`, `TypeVar`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 21,
                            },
                            end: Position {
                                line: 6,
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
            25,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 24;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Semicolon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 25,
                            },
                            end: Position {
                                line: 6,
                                character: 26,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
]
```