# Binary Opr

## Test#0

input

```husky
+
```

output

```husky
TokenSheet {
    tokens: [
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
    ],
    group_starts: [
        0,
    ],
}
```

## Test#1

input

```husky
-
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:2),
            kind: Punctuation(
                Minus,
            ),
        },
    ],
    group_starts: [
        0,
    ],
}
```

## Test#2

input

```husky
*
```

output

```husky
TokenSheet {
    tokens: [
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
    ],
    group_starts: [
        0,
    ],
}
```

## Test#3

input

```husky
/
```

output

```husky
TokenSheet {
    tokens: [
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
    ],
    group_starts: [
        0,
    ],
}
```

## Test#4

input

```husky
**
```

output

```husky
TokenSheet {
    tokens: [
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
    ],
    group_starts: [
        0,
    ],
}
```

## Test#5

input

```husky
|
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:2),
            kind: Punctuation(
                Vertical,
            ),
        },
    ],
    group_starts: [
        0,
    ],
}
```

## Test#6

input

```husky
||
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:3),
            kind: Punctuation(
                DoubleVertical,
            ),
        },
    ],
    group_starts: [
        0,
    ],
}
```

## Test#7

input

```husky
&&
```

output

```husky
TokenSheet {
    tokens: [
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
    ],
    group_starts: [
        0,
    ],
}
```

## Test#8

input

```husky
==
```

output

```husky
TokenSheet {
    tokens: [
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
    ],
    group_starts: [
        0,
    ],
}
```

## Test#9

input

```husky
!=
```

output

```husky
TokenSheet {
    tokens: [
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
    ],
    group_starts: [
        0,
    ],
}
```

## Test#10

input

```husky
>
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:2),
            kind: Punctuation(
                RAngle,
            ),
        },
    ],
    group_starts: [
        0,
    ],
}
```

## Test#11

input

```husky
>=
```

output

```husky
TokenSheet {
    tokens: [
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
    ],
    group_starts: [
        0,
    ],
}
```

## Test#12

input

```husky
<
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:2),
            kind: Punctuation(
                LAngle,
            ),
        },
    ],
    group_starts: [
        0,
    ],
}
```

## Test#13

input

```husky
<=
```

output

```husky
TokenSheet {
    tokens: [
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
    ],
    group_starts: [
        0,
    ],
}
```
