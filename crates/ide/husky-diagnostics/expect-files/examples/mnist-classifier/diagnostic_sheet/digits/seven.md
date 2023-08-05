DiagnosticSheet {
    [salsa id]: 32,
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
                message: "Type Error: type path mismatch in coersing `f32` into `Option f32` of contract `move `, expected `core::option::Option`, but got `core::num::f32` instead",
                severity: Error,
                range: [18:5, 18:15),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `f32` into `Option f32` of contract `move `, expected `core::option::Option`, but got `core::num::f32` instead",
                severity: Error,
                range: [15:5, 18:15),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [36:17, 36:46),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [39:31, 39:60),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [47:13, 47:43),
            },
        ],
    },
}