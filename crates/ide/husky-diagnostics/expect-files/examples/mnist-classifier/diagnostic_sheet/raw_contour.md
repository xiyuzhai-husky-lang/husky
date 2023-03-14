DiagnosticSheet {
    [salsa id]: 42,
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
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [101:28, 101:86),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [161:13, 161:26),
            },
            Diagnostic {
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [162:14, 162:25),
            },
            Diagnostic {
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [163:14, 163:25),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [261:21, 261:37),
            },
            Diagnostic {
                message: "expected 2 argument, found 0",
                severity: Error,
                range: [180:33, 180:42),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo",
                severity: Error,
                range: [170:22, 170:24),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo",
                severity: Error,
                range: [180:31, 180:33),
            },
            Diagnostic {
                message: "expected 0 argument, found 2",
                severity: Error,
                range: [27:16, 30:10),
            },
        ],
    },
}