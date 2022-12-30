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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
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
        kind: Punctuation(
            BinaryOpr(
                Comparison(
                    Leq,
                ),
            ),
        ),
    },
]
```
