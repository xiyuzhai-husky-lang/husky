DiagnosticSheet {
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
                range: [171:35, 171:47),
            },
            Diagnostic {
                message: "entity tree error NoSubentity",
                severity: Error,
                range: [219:47, 219:61),
            },
            Diagnostic {
                message: "entity tree error NoSubentity",
                severity: Error,
                range: [13:28, 13:31),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}