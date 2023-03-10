DiagnosticSheet {
    [salsa id]: 41,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expected identifier",
                severity: Error,
                range: [52:11, 52:12),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [53:9, 53:41),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [56:1, 56:5),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [57:1, 57:7),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [58:1, 58:7),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [59:1, 59:8),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [171:35, 171:47),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [219:47, 219:61),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [225:52, 225:66),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [232:52, 232:66),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [236:47, 236:61),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [13:28, 13:31),
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
                message: "Type Error: AmbiguateListExpr",
                severity: Error,
                range: [170:22, 170:24),
            },
            Diagnostic {
                message: "Type Error: AmbiguateListExpr",
                severity: Error,
                range: [180:31, 180:33),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [261:21, 261:37),
            },
            Diagnostic {
                message: "expected 0 argument, found 2",
                severity: Error,
                range: [27:16, 30:10),
            },
        ],
    },
}