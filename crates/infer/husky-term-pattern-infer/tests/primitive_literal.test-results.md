# Primitive Literal

## Test#0

input

```husky
1
```

output

```husky
raw_expr_arena = Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    Integer(
                        1,
                    ),
                ),
            ),
            range: [1:1, 1:2),
            base_scope_result: None,
        },
    ],
};

ty_infer_sheet = TermPatternInferSheet {
    term_patt_itr: TermPatternInterner {
        patterns: [],
        unresolved_registry: UnresolvedTermRegistry {
            terms: [
                IntegerLiteral(0),
                IntegerType(UnresolvedTermIdx(0)),
            ],
        },
    },
    var_results: {},
    expr_results: ArenaMap {
        data: [
            Some(
                ExprTermPatternInferResults {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Unresolved(
                            UnresolvedTermIdx(
                                1,
                            ),
                        ),
                    ),
                },
            ),
        ],
    },
};
```

## Test#1

input

```husky
1i32
```

output

```husky
raw_expr_arena = Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    I32(
                        1,
                    ),
                ),
            ),
            range: [1:1, 1:5),
            base_scope_result: None,
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
    var_results: {},
    expr_results: ArenaMap {
        data: [
            Some(
                ExprTermPatternInferResults {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Resolved(
                                    Atom(Literal(I32(1))),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Resolved(
                            Atom(Entity { path: `i32` }),
                        ),
                    ),
                },
            ),
        ],
    },
};
```

## Test#2

input

```husky
1i64
```

output

```husky
raw_expr_arena = Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    I64(
                        1,
                    ),
                ),
            ),
            range: [1:1, 1:5),
            base_scope_result: None,
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
    var_results: {},
    expr_results: ArenaMap {
        data: [
            Some(
                ExprTermPatternInferResults {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Resolved(
                                    Atom(Literal(I64(1))),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Resolved(
                            Atom(Entity { path: `i32` }),
                        ),
                    ),
                },
            ),
        ],
    },
};
```

## Test#3

input

```husky
1.
```

output

```husky
raw_expr_arena = Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    Float(
                        OrderedFloat(
                            1.0,
                        ),
                    ),
                ),
            ),
            range: [1:1, 1:3),
            base_scope_result: None,
        },
    ],
};

ty_infer_sheet = TermPatternInferSheet {
    term_patt_itr: TermPatternInterner {
        patterns: [],
        unresolved_registry: UnresolvedTermRegistry {
            terms: [
                FloatLiteral(
                    0,
                ),
                FloatType(
                    UnresolvedTermIdx(
                        0,
                    ),
                ),
            ],
        },
    },
    var_results: {},
    expr_results: ArenaMap {
        data: [
            Some(
                ExprTermPatternInferResults {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Unresolved(
                            UnresolvedTermIdx(
                                1,
                            ),
                        ),
                    ),
                },
            ),
        ],
    },
};
```
