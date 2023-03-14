DiagnosticSheet {
    [salsa id]: 40,
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
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [161:48, 161:51),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [169:60, 169:63),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [173:45, 173:48),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [184:45, 184:48),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
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
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [64:13, 64:37),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [72:13, 72:37),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [81:18, 81:49),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [82:13, 82:29),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [85:9, 85:14),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [86:14, 86:45),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [107:9, 107:14),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [108:14, 108:45),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [114:19, 114:50),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [115:27, 115:43),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [117:9, 117:16),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [118:15, 118:46),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [125:18, 125:49),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [145:13, 145:20),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [158:23, 158:39),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [168:25, 168:41),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [190:13, 190:29),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo",
                severity: Error,
                range: [155:29, 155:31),
            },
            Diagnostic {
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [23:27, 23:63),
            },
        ],
    },
}