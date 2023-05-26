DiagnosticSheet {
    [salsa id]: 44,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [172:35, 172:47),
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
                range: [102:74, 102:83),
            },
            Diagnostic {
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [163:14, 163:25),
            },
            Diagnostic {
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [164:14, 164:25),
            },
            Diagnostic {
                message: "Type Error: no method named `lastx` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [219:33, 219:38),
            },
            Diagnostic {
                message: "Type Error: no method named `lastx` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [226:33, 226:38),
            },
            Diagnostic {
                message: "Type Error: no method named `lastx` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [233:33, 233:38),
            },
            Diagnostic {
                message: "Type Error: no method named `popx` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [264:25, 264:29),
            },
            Diagnostic {
                message: "OriginalFluffyTermExpectationError::Todo",
                severity: Error,
                range: [171:22, 171:24),
            },
        ],
    },
}