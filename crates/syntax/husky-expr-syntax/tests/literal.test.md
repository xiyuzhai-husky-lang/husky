
input

```husky
0
```

output

```husky
Arena ([
  #0: RawExpr {
    variant: Atom(
        Literal(
            Integer(
                0,
            ),
        ),
    ),
    range: [1:1, 1:2),
    base_scope_result: None,
}
])
```
input

```husky
1
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
])
```
input

```husky
-1
```

output

```husky
Arena ([
  #0: RawExpr {
    variant: Atom(
        Literal(
            Integer(
                -1,
            ),
        ),
    ),
    range: [1:2, 1:3),
    base_scope_result: None,
}
])
```
input

```husky
1i32
```

output

```husky
Arena ([
  #0: RawExpr {
    variant: Atom(
        Literal(
            I32(
                1,
            ),
        ),
    ),
    range: [1:1, 1:5),
    base_scope_result: None,
}
])
```
input

```husky
2i64
```

output

```husky
Arena ([
  #0: RawExpr {
    variant: Atom(
        Literal(
            I64(
                2,
            ),
        ),
    ),
    range: [1:1, 1:5),
    base_scope_result: None,
}
])
```
input

```husky
1b32
```

output

```husky
Arena ([
  #0: RawExpr {
    variant: Atom(
        Literal(
            B32(
                1,
            ),
        ),
    ),
    range: [1:1, 1:5),
    base_scope_result: None,
}
])
```
input

```husky
1b64
```

output

```husky
Arena ([
  #0: RawExpr {
    variant: Atom(
        Literal(
            B64(
                1,
            ),
        ),
    ),
    range: [1:1, 1:5),
    base_scope_result: None,
}
])
```
input

```husky
0.1
```

output

```husky
Arena ([
  #0: RawExpr {
    variant: Atom(
        Literal(
            Float(
                OrderedFloat(
                    0.1,
                ),
            ),
        ),
    ),
    range: [1:1, 1:4),
    base_scope_result: None,
}
])
```