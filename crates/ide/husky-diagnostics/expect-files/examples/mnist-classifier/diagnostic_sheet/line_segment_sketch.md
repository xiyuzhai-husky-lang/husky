DiagnosticSheet {
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expected identifier",
                severity: Error,
                range: [28:11, 28:12),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [29:9, 29:58),
            },
            Diagnostic {
                message: "Syntax Error: expected identifier",
                severity: Error,
                range: [60:11, 60:12),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [61:9, 61:21),
            },
            Diagnostic {
                message: "Syntax Error: standalone elif",
                severity: Error,
                range: [98:9, 98:30),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [99:13, 99:28),
            },
            Diagnostic {
                message: "Syntax Error: standalone elif",
                severity: Error,
                range: [129:9, 129:30),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [130:13, 130:28),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "entity tree error NoSubentity",
                severity: Error,
                range: [161:48, 161:51),
            },
            Diagnostic {
                message: "entity tree error NoSubentity",
                severity: Error,
                range: [169:60, 169:63),
            },
            Diagnostic {
                message: "entity tree error NoSubentity",
                severity: Error,
                range: [173:45, 173:48),
            },
            Diagnostic {
                message: "entity tree error NoSubentity",
                severity: Error,
                range: [184:45, 184:48),
            },
            Diagnostic {
                message: "entity tree error NoSubentity",
                severity: Error,
                range: [195:53, 195:56),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}