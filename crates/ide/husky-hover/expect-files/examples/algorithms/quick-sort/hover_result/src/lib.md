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
                            value: "Other keyword\n\n",
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
                            value: "\n\nbox colon\n\ncoercion = None\n\ntype = `Type @ None -> Type @ None @ None`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 40,
                            },
                            end: Position {
                                line: 0,
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
            29,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\ncall par\n\ncoercion = None\n\ntype = `unit @ None`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 2,
                                character: 18,
                            },
                            end: Position {
                                line: 2,
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
            43,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nentity node",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 3,
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
            57,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nvariable\n\ncoercion = None\n\ntype = `Type @ Some(Compterm)`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 42,
                            },
                            end: Position {
                                line: 4,
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
            71,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nvariable\n\ncoercion = Some(\n    Trivial(\n        TrivialFlyCoercion {\n            expectee_quary: StackPure {\n                place: Idx(\n                    PlaceIdx(2),\n                ),\n            },\n        },\n    ),\n)\n\ntype = `isize @ Some(StackPure { place: Idx(PlaceIdx(2)) })`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 13,
                            },
                            end: Position {
                                line: 5,
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
            85,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\ncall par\n\ncoercion = Some(\n    Trivial(\n        TrivialFlyCoercion {\n            expectee_quary: Transient,\n        },\n    ),\n)\n\ntype = `unit @ None`",
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
            99,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 8,
                                character: 30,
                            },
                            end: Position {
                                line: 8,
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
            113,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other keyword\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 25,
                            },
                            end: Position {
                                line: 10,
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
            127,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nentity `core::num::isize`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 58,
                            },
                            end: Position {
                                line: 10,
                                character: 63,
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
            141,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 12,
                                character: 24,
                            },
                            end: Position {
                                line: 12,
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
            155,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nliteral\n\ncoercion = Some(\n    Trivial(\n        TrivialFlyCoercion {\n            expectee_quary: Compterm,\n        },\n    ),\n)\n\ntype = `isize @ Some(Compterm)`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 16,
                                character: 23,
                            },
                            end: Position {
                                line: 16,
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
            169,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nvariable\n\ncoercion = None\n\ntype = `isize @ Some(MutableOnStack { place: Idx(PlaceIdx(1)) })`",
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
            183,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 20,
                                character: 48,
                            },
                            end: Position {
                                line: 20,
                                character: 50,
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
            197,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 22,
                                character: 23,
                            },
                            end: Position {
                                line: 22,
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
            211,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nvariable\n\ncoercion = None\n\ntype = `isize @ Some(MutableOnStack { place: Idx(PlaceIdx(2)) })`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 25,
                                character: 43,
                            },
                            end: Position {
                                line: 25,
                                character: 53,
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
                            value: "\n\nentity `core::num::usize`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 26,
                                character: 44,
                            },
                            end: Position {
                                line: 26,
                                character: 49,
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
            239,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nvec functor box prefix\n\ncoercion = None\n\ntype = `Type @ None -> Type @ None @ None`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 31,
                                character: 15,
                            },
                            end: Position {
                                line: 31,
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
            253,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nliteral\n\ncoercion = Some(\n    Trivial(\n        TrivialFlyCoercion {\n            expectee_quary: Compterm,\n        },\n    ),\n)\n\ntype = `i32 @ Some(Compterm)`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 31,
                                character: 39,
                            },
                            end: Position {
                                line: 31,
                                character: 40,
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
            267,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "This is a form keyword\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 36,
                                character: 0,
                            },
                            end: Position {
                                line: 36,
                                character: 2,
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
            281,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\n\nliteral\n\ncoercion = Some(\n    Trivial(\n        TrivialFlyCoercion {\n            expectee_quary: Compterm,\n        },\n    ),\n)\n\ntype = `Ref 'static str @ Some(Compterm)`",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 37,
                                character: 38,
                            },
                            end: Position {
                                line: 37,
                                character: 48,
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