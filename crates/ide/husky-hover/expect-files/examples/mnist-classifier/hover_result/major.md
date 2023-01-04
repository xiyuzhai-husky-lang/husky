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
                7,
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
                                    character: 4,
                                },
                                end: Position {
                                    line: 1,
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
                14,
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
                                    line: 2,
                                    character: 9,
                                },
                                end: Position {
                                    line: 2,
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
                21,
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
                                    character: 11,
                                },
                                end: Position {
                                    line: 3,
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
                28,
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
                                    line: 5,
                                    character: 26,
                                },
                                end: Position {
                                    line: 5,
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
                35,
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
                                    character: 30,
                                },
                                end: Position {
                                    line: 6,
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
                42,
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
                                    line: 9,
                                    character: 4,
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
                49,
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
                                    line: 10,
                                    character: 12,
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
                56,
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
                                    character: 32,
                                },
                                end: Position {
                                    line: 11,
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
                63,
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
                                    line: 12,
                                    character: 25,
                                },
                                end: Position {
                                    line: 12,
                                    character: 26,
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
                                value: "\n        Bind a value to a variable.\n\nThe primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.\n\nlet thing1: i32 = 100;\nlet thing2 = 200 + thing1;\n\nlet mut changing_thing = true;\nchanging_thing = false;\n\nlet (part1, part2) = (\"first\", \"second\");\n\nstruct Example {\n    a: bool,\n    b: u64,\n}\n\nlet Example { a, b: _ } = Example {\n    a: true,\n    b: 10004,\n};\nassert!(a);\nThe pattern is most commonly a single variable, which means no pattern matching is done and the expression given is bound to the variable. Apart from that, patterns used in let bindings can be as complicated as needed, given that the pattern is exhaustive. See the Rust book for more information on pattern matching. The type of the pattern is optionally given afterwards, but if left blank is automatically inferred by the compiler if possible.\n\nVariables in Rust are immutable by default, and require the mut keyword to be made mutable.\n\nMultiple variables can be defined with the same name, known as shadowing. This doesn't affect the original variable in any way beyond being unable to directly access it beyond the point of shadowing. It continues to remain in scope, getting dropped only when it falls out of scope. Shadowed variables don't need to have the same type as the variables shadowing them.\n\nlet shadowing_example = true;\nlet shadowing_example = 123.4;\nlet shadowing_example = shadowing_example as u32;\nlet mut shadowing_example = format!(\"cool! {shadowing_example}\");\nshadowing_example += \" something else!\"; // not shadowing\nOther places the let keyword is used include along with if, in the form of if let expressions. They're useful if the pattern being matched isn't exhaustive, such as with enumerations. while let also exists, which runs a loop with a pattern matched value until that pattern can't be matched.\n\nFor more information on the let keyword, see the Rust book or the Reference",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 8,
                                },
                                end: Position {
                                    line: 13,
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
                77,
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
                                    character: 31,
                                },
                                end: Position {
                                    line: 14,
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
                84,
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
                                    character: 32,
                                },
                                end: Position {
                                    line: 16,
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
                91,
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
                                    line: 19,
                                    character: 4,
                                },
                                end: Position {
                                    line: 19,
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
                98,
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
                                    line: 20,
                                    character: 10,
                                },
                                end: Position {
                                    line: 20,
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
                105,
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
                                    line: 21,
                                    character: 8,
                                },
                                end: Position {
                                    line: 21,
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
                112,
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
                                    line: 21,
                                    character: 39,
                                },
                                end: Position {
                                    line: 21,
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
                119,
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
                                    line: 24,
                                    character: 0,
                                },
                                end: Position {
                                    line: 24,
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
                                    line: 25,
                                    character: 4,
                                },
                                end: Position {
                                    line: 25,
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
                133,
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
                                    line: 28,
                                    character: 26,
                                },
                                end: Position {
                                    line: 28,
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
                                    line: 29,
                                    character: 44,
                                },
                                end: Position {
                                    line: 29,
                                    character: 45,
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
                147,
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
                                    line: 32,
                                    character: 21,
                                },
                                end: Position {
                                    line: 32,
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
                154,
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
                                    line: 34,
                                    character: 35,
                                },
                                end: Position {
                                    line: 34,
                                    character: 51,
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