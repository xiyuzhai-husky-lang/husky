DiagnosticSheet {
    [salsa id]: 22,
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
                range: [114:11, 114:12),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [115:9, 115:13),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [22:11, 22:24),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [125:39, 125:52),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [132:28, 132:41),
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
                range: [16:15, 16:29),
            },
            Diagnostic {
                message: "Type Error: no method named `push` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(core::mem::Leash) TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [34:17, 34:21),
            },
            Diagnostic {
                message: "Type Error: no method named `push` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(core::mem::Leash) TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [35:17, 35:21),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got core::mem::Leash instead",
                severity: Error,
                range: [36:25, 36:32),
            },
        ],
    },
}