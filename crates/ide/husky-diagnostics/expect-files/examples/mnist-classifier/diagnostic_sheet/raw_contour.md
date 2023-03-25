DiagnosticSheet {
    [salsa id]: 42,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "unresolved identifier",
                severity: Error,
                range: [4:13, 4:26),
            },
            Diagnostic {
                message: "unresolved identifier",
                severity: Error,
                range: [4:28, 4:40),
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
                message: "Type Error: no method named `last_bits` for type `LocalTerm::Resolved(Term(`TypeOntology(core::raw_bits::r32)`))`",
                severity: Error,
                range: [101:74, 101:83),
            },
            Diagnostic {
                message: "Type Error: no method named `ilen` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [161:20, 161:24),
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
                message: "Type Error: no method named `lastx` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [218:33, 218:38),
            },
            Diagnostic {
                message: "Type Error: no method named `push` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [219:33, 219:37),
            },
            Diagnostic {
                message: "Type Error: no method named `lastx` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [225:33, 225:38),
            },
            Diagnostic {
                message: "Type Error: no method named `lastx` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [232:33, 232:38),
            },
            Diagnostic {
                message: "Type Error: no method named `push` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [236:33, 236:37),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [261:21, 261:37),
            },
            Diagnostic {
                message: "Type Error: no method named `popx` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [263:25, 263:29),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo",
                severity: Error,
                range: [170:22, 170:24),
            },
        ],
    },
}