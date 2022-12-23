# Binary Opr

## Test#0

input

```husky
+
```

output

```husky
[
    Token {
        range: [1:1, 1:2),
        kind: Special(
            BinaryOpr(
                PureClosed(
                    Add,
                ),
            ),
        ),
    },
]
```

## Test#1

input

```husky
-
```

output

```husky
[
    Token {
        range: [1:1, 1:2),
        kind: Special(
            Minus,
        ),
    },
]
```

## Test#2

input

```husky
*
```

output

```husky
[
    Token {
        range: [1:1, 1:2),
        kind: Special(
            BinaryOpr(
                PureClosed(
                    Mul,
                ),
            ),
        ),
    },
]
```

## Test#3

input

```husky
/
```

output

```husky
[
    Token {
        range: [1:1, 1:2),
        kind: Special(
            BinaryOpr(
                PureClosed(
                    Div,
                ),
            ),
        ),
    },
]
```

## Test#4

input

```husky
**
```

output

```husky
[
    Token {
        range: [1:1, 1:3),
        kind: Special(
            BinaryOpr(
                PureClosed(
                    Power,
                ),
            ),
        ),
    },
]
```

## Test#5

input

```husky
|
```

output

```husky
[
    Token {
        range: [1:1, 1:2),
        kind: Special(
            Vertical,
        ),
    },
]
```

## Test#6

input

```husky
||
```

output

```husky
[
    Token {
        range: [1:1, 1:3),
        kind: Special(
            DoubleVertical,
        ),
    },
]
```

## Test#7

input

```husky
&&
```

output

```husky
[
    Token {
        range: [1:1, 1:3),
        kind: Special(
            BinaryOpr(
                ShortcuitLogic(
                    And,
                ),
            ),
        ),
    },
]
```

## Test#8

input

```husky
==
```

output

```husky
[
    Token {
        range: [1:1, 1:3),
        kind: Special(
            BinaryOpr(
                Comparison(
                    Eq,
                ),
            ),
        ),
    },
]
```

## Test#9

input

```husky
!=
```

output

```husky
[
    Token {
        range: [1:1, 1:3),
        kind: Special(
            BinaryOpr(
                Comparison(
                    Neq,
                ),
            ),
        ),
    },
]
```

## Test#10

input

```husky
>
```

output

```husky
[
    Token {
        range: [1:1, 1:2),
        kind: Special(
            RAngle,
        ),
    },
]
```

## Test#11

input

```husky
>=
```

output

```husky
[
    Token {
        range: [1:1, 1:3),
        kind: Special(
            BinaryOpr(
                Comparison(
                    Geq,
                ),
            ),
        ),
    },
]
```

## Test#12

input

```husky
<
```

output

```husky
[
    Token {
        range: [1:1, 1:2),
        kind: Special(
            LAngle,
        ),
    },
]
```

## Test#13

input

```husky
<=
```

output

```husky
[
    Token {
        range: [1:1, 1:3),
        kind: Special(
            BinaryOpr(
                Comparison(
                    Leq,
                ),
            ),
        ),
    },
]
```
