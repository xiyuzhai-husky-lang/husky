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
                            value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Keyword(\n    Keyword::Mod,\n);\n\ntoken_info = None;\n\n\n",
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
                            value: "\ntoken_idx = 1;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Ident(\n    `major_item`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::Submodule(\n                Room32,\n                SubmoduleSynNodePath(\n                    ItemSynNodePathId {\n                        data: ItemSynNodePathData::Submodule(\n                            SubmoduleSynNodePathData {\n                                maybe_ambiguous_path: MaybeAmbiguousPath {\n                                    path: SubmoduleItemPath(\n                                        ItemPathId(\n                                            Id {\n                                                value: 6,\n                                            },\n                                        ),\n                                    ),\n                                    disambiguator: 0,\n                                },\n                            },\n                        ),\n                    },\n                ),\n            ),\n            EntityKind::Module,\n        ),\n    },\n);\n\n\n",
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
                                character: 14,
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