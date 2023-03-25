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
                message: "Type Error: no method named `sqrt` for type `LocalTerm::Resolved(Term(`TypeOntology(core::num::f32)`))`",
                severity: Error,
                range: [64:31, 64:35),
            },
            Diagnostic {
                message: "Type Error: no method named `sqrt` for type `LocalTerm::Resolved(Term(`TypeOntology(core::num::f32)`))`",
                severity: Error,
                range: [72:31, 72:35),
            },
            Diagnostic {
                message: "Type Error: no method named `ilen` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [82:23, 82:27),
            },
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
                message: "Type Error: no method named `ilen` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [115:37, 115:41),
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
                message: "Type Error: no method named `ilen` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [158:33, 158:37),
            },
            Diagnostic {
                message: "Type Error: no method named `ilen` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [168:35, 168:39),
            },
            Diagnostic {
                message: "Type Error: no method named `ilen` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [190:23, 190:27),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo",
                severity: Error,
                range: [155:29, 155:31),
            },
            Diagnostic {
                message: "Type Error: no method named `cyclic_slice` for type `LocalTerm::Resolved(Term(`TypeOntology(core::list::List) TypeOntology(mnist_classifier::geom2d::Point2d)`))`",
                severity: Error,
                range: [23:37, 23:49),
            },
        ],
    },
}