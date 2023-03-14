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
                message: "Type Error: no method named `displacement` for type `LocalTerm::Resolved(Term(`TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [81:21, 81:33),
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
                message: "Type Error: no method named `displacement` for type `LocalTerm::Resolved(Term(`TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [86:17, 86:29),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [107:9, 107:14),
            },
            Diagnostic {
                message: "Type Error: no method named `displacement` for type `LocalTerm::Resolved(Term(`TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [108:17, 108:29),
            },
            Diagnostic {
                message: "Type Error: no method named `displacement` for type `LocalTerm::Resolved(Term(`TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [114:22, 114:34),
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
                message: "Type Error: no method named `displacement` for type `LocalTerm::Resolved(Term(`TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [118:18, 118:30),
            },
            Diagnostic {
                message: "Type Error: no method named `displacement` for type `LocalTerm::Resolved(Term(`TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [125:21, 125:33),
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