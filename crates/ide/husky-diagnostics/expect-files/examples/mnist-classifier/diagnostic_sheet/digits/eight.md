DiagnosticSheet {
    [salsa id]: 27,
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
                range: [21:8, 21:54),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [22:12, 22:58),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `f32` into `Option f32` of contract `move `, expected `core::option::Option`, but got `core::num::f32` instead",
                severity: Error,
                range: [30:5, 30:36),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `f32` into `Option f32` of contract `move `, expected `core::option::Option`, but got `core::num::f32` instead",
                severity: Error,
                range: [28:5, 30:36),
            },
        ],
    },
}