# Variable

## Test#0

input

```husky
x
```

output

```husky
raw_expr_arena = IdxArena {
    data: [
        RawExpr {
            variant: Atom(
                Symbol(
                    Symbol {
                        ident: Custom(
                            CustomIdentifier(
                                "x",
                            ),
                        ),
                        kind: Unrecognized,
                    },
                ),
            ),
            range: [1:1, 1:2),
            base_scope_result: Uncertain,
        },
    ],
};

ty_infer_sheet = TermPatternInferSheet {
    term_patt_itr: TermPatternInterner {
        patterns: [],
        unresolved_registry: UnresolvedTermRegistry {
            terms: [],
        },
    },
    expr_results: ArenaMap {
        data: [
            Some(
                TermPatternInferEntry {
                    const_expr: Err(
                        TermPatternInferError(
                            TermPatternInferErrorInternal {
                                source: Original(
                                    IdentUnrecognized {
                                        ident: Custom(
                                            CustomIdentifier(
                                                "x",
                                            ),
                                        ),
                                    },
                                ),
                                range: [1:1, 1:2),
                            },
                        ),
                    ),
                    ty: Err(
                        TermPatternInferError(
                            TermPatternInferErrorInternal {
                                source: Derived(
                                    TermPatternInferError(
                                        TermPatternInferError(
                                            TermPatternInferErrorInternal {
                                                source: Original(
                                                    IdentUnrecognized {
                                                        ident: Custom(
                                                            CustomIdentifier(
                                                                "x",
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                range: [1:1, 1:2),
                                            },
                                        ),
                                    ),
                                ),
                                range: [1:1, 1:2),
                            },
                        ),
                    ),
                },
            ),
        ],
    },
};
```
