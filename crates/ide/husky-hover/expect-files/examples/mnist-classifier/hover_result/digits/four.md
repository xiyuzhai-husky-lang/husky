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
                20,
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
                                    line: 1,
                                    character: 34,
                                },
                                end: Position {
                                    line: 1,
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
                40,
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
                                    line: 3,
                                    character: 31,
                                },
                                end: Position {
                                    line: 3,
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
                60,
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
                                    line: 6,
                                    character: 49,
                                },
                                end: Position {
                                    line: 6,
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
                80,
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
                                    line: 11,
                                    character: 25,
                                },
                                end: Position {
                                    line: 11,
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
                100,
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
                                    line: 15,
                                    character: 49,
                                },
                                end: Position {
                                    line: 15,
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
                120,
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
                                    line: 19,
                                    character: 57,
                                },
                                end: Position {
                                    line: 19,
                                    character: 58,
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
                140,
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
                                    line: 26,
                                    character: 12,
                                },
                                end: Position {
                                    line: 26,
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
                160,
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
                                    line: 44,
                                    character: 4,
                                },
                                end: Position {
                                    line: 44,
                                    character: 89,
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
                                value: "",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 58,
                                    character: 44,
                                },
                                end: Position {
                                    line: 58,
                                    character: 56,
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
                200,
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
                                    line: 71,
                                    character: 36,
                                },
                                end: Position {
                                    line: 71,
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
                220,
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
                                    line: 76,
                                    character: 4,
                                },
                                end: Position {
                                    line: 76,
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
                240,
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
                                    line: 78,
                                    character: 37,
                                },
                                end: Position {
                                    line: 78,
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
                260,
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
                                    line: 81,
                                    character: 28,
                                },
                                end: Position {
                                    line: 81,
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
                280,
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
                                    line: 84,
                                    character: 8,
                                },
                                end: Position {
                                    line: 84,
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
                                    line: 86,
                                    character: 39,
                                },
                                end: Position {
                                    line: 86,
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
                320,
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
                                    line: 91,
                                    character: 8,
                                },
                                end: Position {
                                    line: 91,
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
                340,
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
                                    line: 105,
                                    character: 33,
                                },
                                end: Position {
                                    line: 105,
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
                360,
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
                                    line: 108,
                                    character: 4,
                                },
                                end: Position {
                                    line: 108,
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
                380,
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
                                    character: 16,
                                },
                                end: Position {
                                    line: 111,
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
                400,
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
                                    line: 115,
                                    character: 4,
                                },
                                end: Position {
                                    line: 115,
                                    character: 29,
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