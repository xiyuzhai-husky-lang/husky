DiagnosticSheet {
    [salsa id]: 31,
    item_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [24:24, 24:50),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [25:22, 25:48),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [26:19, 26:45),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `f32` to `HollowTermTodo` under contract ``",
                severity: Error,
                range: [33:27, 33:63),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `f32` to `HollowTermTodo` under contract ``",
                severity: Error,
                range: [33:66, 33:102),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [45:13, 45:39),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [46:13, 46:39),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [68:17, 68:43),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [69:17, 69:43),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [70:17, 70:43),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `f32` into `Option f32` of contract `move `, expected `core::option::Option`, but got `core::num::f32` instead",
                severity: Error,
                range: [101:5, 101:16),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `f32` into `Option f32` of contract `move `, expected `core::option::Option`, but got `core::num::f32` instead",
                severity: Error,
                range: [99:5, 101:16),
            },
        ],
    },
}