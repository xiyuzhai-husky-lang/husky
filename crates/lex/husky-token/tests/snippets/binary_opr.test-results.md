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
            Binary(
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
            Binary(
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
            Binary(
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
            Binary(
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
            Binary(
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
            Binary(
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
            Binary(
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
            Binary(
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
            Binary(
                Comparison(
                    Leq,
                ),
            ),
        ),
    },
]
```
