DiagnosticSheet {
    [salsa id]: 39,
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
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [15:14, 15:15),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [15:14, 15:15),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [161:48, 161:51),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [169:60, 169:63),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [173:45, 173:48),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [184:45, 184:48),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::NoSubentity",
                severity: Error,
                range: [195:53, 195:56),
            },
        ],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expect `}`",
                severity: Error,
                range: [16:20, 16:22),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [85:9, 85:14),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [107:9, 107:14),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [117:9, 117:16),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [145:13, 145:20),
            },
            Diagnostic {
                message: "Type Error: AmbiguateListExpr",
                severity: Error,
                range: [155:29, 155:31),
            },
            Diagnostic {
                message: "expected 0 argument, found 2",
                severity: Error,
                range: [52:16, 55:10),
            },
        ],
    },
}