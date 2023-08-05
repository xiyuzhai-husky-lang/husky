DiagnosticSheet {
    [salsa id]: 39,
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
                message: "Term Error: expected coersion from `i32 at StackPure { location: StackLocationIdx(LocalSymbolIdx(1)) }` to `HollowTermTodo` under contract ``",
                severity: Error,
                range: [8:23, 8:28),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `i32 at StackPure { location: StackLocationIdx(LocalSymbolIdx(0)) }` to `HollowTermTodo` under contract ``",
                severity: Error,
                range: [8:44, 8:45),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `f32` under contract `move `",
                severity: Error,
                range: [54:17, 54:23),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `f32` under contract `move `",
                severity: Error,
                range: [75:17, 75:23),
            },
        ],
    },
}