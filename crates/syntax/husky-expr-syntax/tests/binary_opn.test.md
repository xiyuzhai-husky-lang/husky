# Binary Opn

## Test#0

input

```husky
1 + 1
```

output

```husky
Arena ([
  #0: RawExpr {
    variant: Atom(
        Literal(
            Integer(
                1,
            ),
        ),
    ),
    range: [1:1, 1:2),
    base_scope_result: None,
}
  #1: RawExpr {
    variant: Atom(
        Literal(
            Integer(
                1,
            ),
        ),
    ),
    range: [1:5, 1:6),
    base_scope_result: None,
}
  #2: RawExpr {
    variant: Opn {
        opn_variant: Binary Pure(
            Add,
        ),
        opds: 0..2,
    },
    range: [1:1, 1:6),
    base_scope_result: None,
}
])
```

## Test#1

input

```husky
1 - 1
```

output

```husky
Arena ([
  #0: RawExpr {
    variant: Atom(
        Literal(
            Integer(
                1,
            ),
        ),
    ),
    range: [1:1, 1:2),
    base_scope_result: None,
}
  #1: RawExpr {
    variant: Atom(
        Literal(
            Integer(
                1,
            ),
        ),
    ),
    range: [1:5, 1:6),
    base_scope_result: None,
}
  #2: RawExpr {
    variant: Opn {
        opn_variant: Binary Pure(
            Sub,
        ),
        opds: 0..2,
    },
    range: [1:1, 1:6),
    base_scope_result: None,
}
])
```
