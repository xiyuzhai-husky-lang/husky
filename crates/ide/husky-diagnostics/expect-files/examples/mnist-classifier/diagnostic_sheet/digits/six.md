DiagnosticSheet {
    [salsa id]: 33,
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
                range: [11:24, 11:44),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [21:29, 21:58),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `f32` into `Option f32` of contract `move `, expected `core::option::Option`, but got `core::num::f32` instead",
                severity: Error,
                range: [74:5, 74:16),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `f32` into `Option f32` of contract `move `, expected `core::option::Option`, but got `core::num::f32` instead",
                severity: Error,
                range: [68:5, 74:16),
            },
        ],
    },
}