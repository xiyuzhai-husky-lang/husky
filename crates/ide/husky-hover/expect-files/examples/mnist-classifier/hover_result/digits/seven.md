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
                                value: "\n        Bind a value to a variable.\n\nThe primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.\n\nlet thing1: i32 = 100;\nlet thing2 = 200 + thing1;\n\nlet mut changing_thing = true;\nchanging_thing = false;\n\nlet (part1, part2) = (\"first\", \"second\");\n\nstruct Example {\n    a: bool,\n    b: u64,\n}\n\nlet Example { a, b: _ } = Example {\n    a: true,\n    b: 10004,\n};\nassert!(a);\nThe pattern is most commonly a single variable, which means no pattern matching is done and the expression given is bound to the variable. Apart from that, patterns used in let bindings can be as complicated as needed, given that the pattern is exhaustive. See the Rust book for more information on pattern matching. The type of the pattern is optionally given afterwards, but if left blank is automatically inferred by the compiler if possible.\n\nVariables in Rust are immutable by default, and require the mut keyword to be made mutable.\n\nMultiple variables can be defined with the same name, known as shadowing. This doesn't affect the original variable in any way beyond being unable to directly access it beyond the point of shadowing. It continues to remain in scope, getting dropped only when it falls out of scope. Shadowed variables don't need to have the same type as the variables shadowing them.\n\nlet shadowing_example = true;\nlet shadowing_example = 123.4;\nlet shadowing_example = shadowing_example as u32;\nlet mut shadowing_example = format!(\"cool! {shadowing_example}\");\nshadowing_example += \" something else!\"; // not shadowing\nOther places the let keyword is used include along with if, in the form of if let expressions. They're useful if the pattern being matched isn't exhaustive, such as with enumerations. while let also exists, which runs a loop with a pattern matched value until that pattern can't be matched.\n\nFor more information on the let keyword, see the Rust book or the Reference",
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
                18,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\n        Bind a value to a variable.\n\nThe primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.\n\nlet thing1: i32 = 100;\nlet thing2 = 200 + thing1;\n\nlet mut changing_thing = true;\nchanging_thing = false;\n\nlet (part1, part2) = (\"first\", \"second\");\n\nstruct Example {\n    a: bool,\n    b: u64,\n}\n\nlet Example { a, b: _ } = Example {\n    a: true,\n    b: 10004,\n};\nassert!(a);\nThe pattern is most commonly a single variable, which means no pattern matching is done and the expression given is bound to the variable. Apart from that, patterns used in let bindings can be as complicated as needed, given that the pattern is exhaustive. See the Rust book for more information on pattern matching. The type of the pattern is optionally given afterwards, but if left blank is automatically inferred by the compiler if possible.\n\nVariables in Rust are immutable by default, and require the mut keyword to be made mutable.\n\nMultiple variables can be defined with the same name, known as shadowing. This doesn't affect the original variable in any way beyond being unable to directly access it beyond the point of shadowing. It continues to remain in scope, getting dropped only when it falls out of scope. Shadowed variables don't need to have the same type as the variables shadowing them.\n\nlet shadowing_example = true;\nlet shadowing_example = 123.4;\nlet shadowing_example = shadowing_example as u32;\nlet mut shadowing_example = format!(\"cool! {shadowing_example}\");\nshadowing_example += \" something else!\"; // not shadowing\nOther places the let keyword is used include along with if, in the form of if let expressions. They're useful if the pattern being matched isn't exhaustive, such as with enumerations. while let also exists, which runs a loop with a pattern matched value until that pattern can't be matched.\n\nFor more information on the let keyword, see the Rust book or the Reference",
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
                36,
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
                                    line: 4,
                                    character: 9,
                                },
                                end: Position {
                                    line: 4,
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
                54,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\n        Bind a value to a variable.\n\nThe primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.\n\nlet thing1: i32 = 100;\nlet thing2 = 200 + thing1;\n\nlet mut changing_thing = true;\nchanging_thing = false;\n\nlet (part1, part2) = (\"first\", \"second\");\n\nstruct Example {\n    a: bool,\n    b: u64,\n}\n\nlet Example { a, b: _ } = Example {\n    a: true,\n    b: 10004,\n};\nassert!(a);\nThe pattern is most commonly a single variable, which means no pattern matching is done and the expression given is bound to the variable. Apart from that, patterns used in let bindings can be as complicated as needed, given that the pattern is exhaustive. See the Rust book for more information on pattern matching. The type of the pattern is optionally given afterwards, but if left blank is automatically inferred by the compiler if possible.\n\nVariables in Rust are immutable by default, and require the mut keyword to be made mutable.\n\nMultiple variables can be defined with the same name, known as shadowing. This doesn't affect the original variable in any way beyond being unable to directly access it beyond the point of shadowing. It continues to remain in scope, getting dropped only when it falls out of scope. Shadowed variables don't need to have the same type as the variables shadowing them.\n\nlet shadowing_example = true;\nlet shadowing_example = 123.4;\nlet shadowing_example = shadowing_example as u32;\nlet mut shadowing_example = format!(\"cool! {shadowing_example}\");\nshadowing_example += \" something else!\"; // not shadowing\nOther places the let keyword is used include along with if, in the form of if let expressions. They're useful if the pattern being matched isn't exhaustive, such as with enumerations. while let also exists, which runs a loop with a pattern matched value until that pattern can't be matched.\n\nFor more information on the let keyword, see the Rust book or the Reference",
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
                72,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\n        Bind a value to a variable.\n\nThe primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.\n\nlet thing1: i32 = 100;\nlet thing2 = 200 + thing1;\n\nlet mut changing_thing = true;\nchanging_thing = false;\n\nlet (part1, part2) = (\"first\", \"second\");\n\nstruct Example {\n    a: bool,\n    b: u64,\n}\n\nlet Example { a, b: _ } = Example {\n    a: true,\n    b: 10004,\n};\nassert!(a);\nThe pattern is most commonly a single variable, which means no pattern matching is done and the expression given is bound to the variable. Apart from that, patterns used in let bindings can be as complicated as needed, given that the pattern is exhaustive. See the Rust book for more information on pattern matching. The type of the pattern is optionally given afterwards, but if left blank is automatically inferred by the compiler if possible.\n\nVariables in Rust are immutable by default, and require the mut keyword to be made mutable.\n\nMultiple variables can be defined with the same name, known as shadowing. This doesn't affect the original variable in any way beyond being unable to directly access it beyond the point of shadowing. It continues to remain in scope, getting dropped only when it falls out of scope. Shadowed variables don't need to have the same type as the variables shadowing them.\n\nlet shadowing_example = true;\nlet shadowing_example = 123.4;\nlet shadowing_example = shadowing_example as u32;\nlet mut shadowing_example = format!(\"cool! {shadowing_example}\");\nshadowing_example += \" something else!\"; // not shadowing\nOther places the let keyword is used include along with if, in the form of if let expressions. They're useful if the pattern being matched isn't exhaustive, such as with enumerations. while let also exists, which runs a loop with a pattern matched value until that pattern can't be matched.\n\nFor more information on the let keyword, see the Rust book or the Reference",
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
                90,
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
                                    line: 14,
                                    character: 66,
                                },
                                end: Position {
                                    line: 14,
                                    character: 67,
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
                108,
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
                                    line: 17,
                                    character: 16,
                                },
                                end: Position {
                                    line: 17,
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
                126,
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
                                    line: 22,
                                    character: 4,
                                },
                                end: Position {
                                    line: 22,
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
                144,
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
                                    line: 24,
                                    character: 45,
                                },
                                end: Position {
                                    line: 24,
                                    character: 47,
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
                162,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\n        Bind a value to a variable.\n\nThe primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.\n\nlet thing1: i32 = 100;\nlet thing2 = 200 + thing1;\n\nlet mut changing_thing = true;\nchanging_thing = false;\n\nlet (part1, part2) = (\"first\", \"second\");\n\nstruct Example {\n    a: bool,\n    b: u64,\n}\n\nlet Example { a, b: _ } = Example {\n    a: true,\n    b: 10004,\n};\nassert!(a);\nThe pattern is most commonly a single variable, which means no pattern matching is done and the expression given is bound to the variable. Apart from that, patterns used in let bindings can be as complicated as needed, given that the pattern is exhaustive. See the Rust book for more information on pattern matching. The type of the pattern is optionally given afterwards, but if left blank is automatically inferred by the compiler if possible.\n\nVariables in Rust are immutable by default, and require the mut keyword to be made mutable.\n\nMultiple variables can be defined with the same name, known as shadowing. This doesn't affect the original variable in any way beyond being unable to directly access it beyond the point of shadowing. It continues to remain in scope, getting dropped only when it falls out of scope. Shadowed variables don't need to have the same type as the variables shadowing them.\n\nlet shadowing_example = true;\nlet shadowing_example = 123.4;\nlet shadowing_example = shadowing_example as u32;\nlet mut shadowing_example = format!(\"cool! {shadowing_example}\");\nshadowing_example += \" something else!\"; // not shadowing\nOther places the let keyword is used include along with if, in the form of if let expressions. They're useful if the pattern being matched isn't exhaustive, such as with enumerations. while let also exists, which runs a loop with a pattern matched value until that pattern can't be matched.\n\nFor more information on the let keyword, see the Rust book or the Reference",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 27,
                                    character: 4,
                                },
                                end: Position {
                                    line: 27,
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
                                    line: 30,
                                    character: 5,
                                },
                                end: Position {
                                    line: 30,
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
                198,
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
                                    line: 31,
                                    character: 29,
                                },
                                end: Position {
                                    line: 31,
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
                216,
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
                                    line: 36,
                                    character: 8,
                                },
                                end: Position {
                                    line: 36,
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
                234,
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
                                    line: 40,
                                    character: 4,
                                },
                                end: Position {
                                    line: 40,
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
                252,
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
                                    character: 12,
                                },
                                end: Position {
                                    line: 44,
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
                270,
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
                                    line: 47,
                                    character: 34,
                                },
                                end: Position {
                                    line: 47,
                                    character: 35,
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
                288,
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
                                    line: 49,
                                    character: 11,
                                },
                                end: Position {
                                    line: 49,
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
                306,
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
                                    line: 51,
                                    character: 16,
                                },
                                end: Position {
                                    line: 51,
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
                324,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\n        Bind a value to a variable.\n\nThe primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.\n\nlet thing1: i32 = 100;\nlet thing2 = 200 + thing1;\n\nlet mut changing_thing = true;\nchanging_thing = false;\n\nlet (part1, part2) = (\"first\", \"second\");\n\nstruct Example {\n    a: bool,\n    b: u64,\n}\n\nlet Example { a, b: _ } = Example {\n    a: true,\n    b: 10004,\n};\nassert!(a);\nThe pattern is most commonly a single variable, which means no pattern matching is done and the expression given is bound to the variable. Apart from that, patterns used in let bindings can be as complicated as needed, given that the pattern is exhaustive. See the Rust book for more information on pattern matching. The type of the pattern is optionally given afterwards, but if left blank is automatically inferred by the compiler if possible.\n\nVariables in Rust are immutable by default, and require the mut keyword to be made mutable.\n\nMultiple variables can be defined with the same name, known as shadowing. This doesn't affect the original variable in any way beyond being unable to directly access it beyond the point of shadowing. It continues to remain in scope, getting dropped only when it falls out of scope. Shadowed variables don't need to have the same type as the variables shadowing them.\n\nlet shadowing_example = true;\nlet shadowing_example = 123.4;\nlet shadowing_example = shadowing_example as u32;\nlet mut shadowing_example = format!(\"cool! {shadowing_example}\");\nshadowing_example += \" something else!\"; // not shadowing\nOther places the let keyword is used include along with if, in the form of if let expressions. They're useful if the pattern being matched isn't exhaustive, such as with enumerations. while let also exists, which runs a loop with a pattern matched value until that pattern can't be matched.\n\nFor more information on the let keyword, see the Rust book or the Reference",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 55,
                                    character: 8,
                                },
                                end: Position {
                                    line: 55,
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
                342,
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
                                    character: 12,
                                },
                                end: Position {
                                    line: 58,
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
                                    line: 61,
                                    character: 16,
                                },
                                end: Position {
                                    line: 61,
                                    character: 21,
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