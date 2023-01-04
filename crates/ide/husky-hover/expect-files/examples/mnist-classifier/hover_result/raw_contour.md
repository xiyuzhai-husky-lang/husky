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
                                value: "Bind a value to a variable.\n\nThe primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.\n\nlet thing1: i32 = 100;\nlet thing2 = 200 + thing1;\n\nlet mut changing_thing = true;\nchanging_thing = false;\n\nlet (part1, part2) = (\"first\", \"second\");\n\nstruct Example {\n    a: bool,\n    b: u64,\n}\n\nlet Example { a, b: _ } = Example {\n    a: true,\n    b: 10004,\n};\nassert!(a);\nThe pattern is most commonly a single variable, which means no pattern matching is done and the expression given is bound to the variable. Apart from that, patterns used in let bindings can be as complicated as needed, given that the pattern is exhaustive. See the Rust book for more information on pattern matching. The type of the pattern is optionally given afterwards, but if left blank is automatically inferred by the compiler if possible.\n\nVariables in Rust are immutable by default, and require the mut keyword to be made mutable.\n\nMultiple variables can be defined with the same name, known as shadowing. This doesn't affect the original variable in any way beyond being unable to directly access it beyond the point of shadowing. It continues to remain in scope, getting dropped only when it falls out of scope. Shadowed variables don't need to have the same type as the variables shadowing them.\n\nlet shadowing_example = true;\nlet shadowing_example = 123.4;\nlet shadowing_example = shadowing_example as u32;\nlet mut shadowing_example = format!(\"cool! {shadowing_example}\");\nshadowing_example += \" something else!\"; // not shadowing\nOther places the let keyword is used include along with if, in the form of if let expressions. They're useful if the pattern being matched isn't exhaustive, such as with enumerations. while let also exists, which runs a loop with a pattern matched value until that pattern can't be matched.\n\nFor more information on the let keyword, see the Rust book or the Reference",
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
                75,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 16,
                                    character: 35,
                                },
                                end: Position {
                                    line: 16,
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
                150,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 25,
                                    character: 33,
                                },
                                end: Position {
                                    line: 25,
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
                225,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 38,
                                    character: 26,
                                },
                                end: Position {
                                    line: 38,
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
                300,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 42,
                                    character: 52,
                                },
                                end: Position {
                                    line: 42,
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
                375,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 57,
                                    character: 0,
                                },
                                end: Position {
                                    line: 57,
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
                450,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 67,
                                    character: 19,
                                },
                                end: Position {
                                    line: 67,
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
                525,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 82,
                                    character: 20,
                                },
                                end: Position {
                                    line: 82,
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
                600,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 98,
                                    character: 22,
                                },
                                end: Position {
                                    line: 98,
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
                675,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 111,
                                    character: 4,
                                },
                                end: Position {
                                    line: 111,
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
                750,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 125,
                                    character: 15,
                                },
                                end: Position {
                                    line: 125,
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
                825,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 140,
                                    character: 23,
                                },
                                end: Position {
                                    line: 140,
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
                900,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 157,
                                    character: 39,
                                },
                                end: Position {
                                    line: 157,
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
                975,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 167,
                                    character: 23,
                                },
                                end: Position {
                                    line: 167,
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
                1050,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 174,
                                    character: 80,
                                },
                                end: Position {
                                    line: 174,
                                    character: 81,
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
                1125,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 184,
                                    character: 83,
                                },
                                end: Position {
                                    line: 184,
                                    character: 84,
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
                1200,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 201,
                                    character: 29,
                                },
                                end: Position {
                                    line: 201,
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
                1275,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 216,
                                    character: 37,
                                },
                                end: Position {
                                    line: 216,
                                    character: 44,
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
                1350,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 229,
                                    character: 42,
                                },
                                end: Position {
                                    line: 229,
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
                1425,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 247,
                                    character: 47,
                                },
                                end: Position {
                                    line: 247,
                                    character: 48,
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
                1500,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 263,
                                    character: 12,
                                },
                                end: Position {
                                    line: 263,
                                    character: 18,
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