DiagnosticSheet {
    [salsa id]: 46,
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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [172:35, 172:47),
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
                message: "Type Error: cannot index into type `BinaryImage28`",
                severity: Error,
                range: [174:20, 174:32),
            },
            Diagnostic {
                message: "Type Error: cannot index into type `BinaryImage28`",
                severity: Error,
                range: [175:20, 175:30),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Option Point2d` to `unit` under contract ``",
                severity: Error,
                range: [264:17, 264:30),
            },
        ],
    },
}