# Literal

## Test#0

input

```husky
0
```

output

```husky
Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    Integer(
                        0,
                    ),
                ),
            ),
            range: [1:1, 1:2),
            base_scope_result: None,
        },
    ],
}
```

## Test#1

input

```husky
1
```

output

```husky
Arena {
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
}
```

## Test#2

input

```husky
-1
```

output

```husky
Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    Integer(
                        -1,
                    ),
                ),
            ),
            range: [1:2, 1:3),
            base_scope_result: None,
        },
    ],
}
```

## Test#3

input

```husky
1i32
```

output

```husky
Arena {
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
}
```

## Test#4

input

```husky
2i64
```

output

```husky
Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    I64(
                        2,
                    ),
                ),
            ),
            range: [1:1, 1:5),
            base_scope_result: None,
        },
    ],
}
```

## Test#5

input

```husky
1r32
```

output

```husky
Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    B32(
                        1,
                    ),
                ),
            ),
            range: [1:1, 1:5),
            base_scope_result: None,
        },
    ],
}
```

## Test#6

input

```husky
1b64
```

output

```husky
Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    B64(
                        1,
                    ),
                ),
            ),
            range: [1:1, 1:5),
            base_scope_result: None,
        },
    ],
}
```

## Test#7

input

```husky
0.1
```

output

```husky
Arena {
    data: [
        Expr {
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
        },
    ],
}
```
