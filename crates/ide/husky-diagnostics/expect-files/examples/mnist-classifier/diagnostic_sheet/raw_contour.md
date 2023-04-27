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
                message: "Type Error: no method named `last_bits` for type `FluffyTerm::EntityPath(TermEntityPath::TypeOntology(TypePath(`core::raw_bits::r32`, `Extern`)))`",
                severity: Error,
                range: [101:74, 101:83),
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
                message: "Type Error: no method named `lastx` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [218:33, 218:38),
            },
            Diagnostic {
                message: "Type Error: no method named `push` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [219:33, 219:37),
            },
            Diagnostic {
                message: "Type Error: no method named `lastx` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [225:33, 225:38),
            },
            Diagnostic {
                message: "Type Error: no method named `lastx` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [232:33, 232:38),
            },
            Diagnostic {
                message: "Type Error: no method named `push` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [236:33, 236:37),
            },
            Diagnostic {
                message: "Type Error: no method named `popx` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [263:25, 263:29),
            },
            Diagnostic {
                message: "OriginalFluffyTermExpectationError::Todo",
                severity: Error,
                range: [170:22, 170:24),
            },
        ],
    },
}