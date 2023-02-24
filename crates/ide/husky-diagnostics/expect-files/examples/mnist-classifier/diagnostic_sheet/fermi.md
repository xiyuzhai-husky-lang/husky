DiagnosticSheet {
    [salsa id]: 32,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [33:27, 33:44),
            },
        ],
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
                message: "Type Error: original `todo` in term Ref 'eval List ConcaveComponent",
                severity: Error,
                range: [32:22, 32:40),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term List invariant Ref 'eval ConcaveComponent -> Option f32",
                severity: Error,
                range: [35:13, 35:22),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [38:12, 38:28),
            },
        ],
    },
}