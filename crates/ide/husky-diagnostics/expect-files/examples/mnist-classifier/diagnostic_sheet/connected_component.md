DiagnosticSheet {
    [salsa id]: 22,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "unresolved identifier",
                severity: Error,
                range: [1:5, 1:7),
            },
        ],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expected identifier",
                severity: Error,
                range: [115:11, 115:12),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [116:9, 116:13),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [23:11, 23:24),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [126:39, 126:52),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [133:28, 133:41),
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
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [17:15, 17:29),
            },
            Diagnostic {
                message: "Type Error: no method named `push` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(core::mem::Leash) TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [35:17, 35:21),
            },
            Diagnostic {
                message: "Type Error: no method named `push` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(core::mem::Leash) TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [36:17, 36:21),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got core::mem::Leash instead",
                severity: Error,
                range: [37:25, 37:32),
            },
        ],
    },
}