# Binary Opr

## Test#0

input

```husky
+
```

output

```husky
RangedTokenSheet {
    tokens: [
        Punctuation(
            Binary(
                PureClosed(
                    Add,
                ),
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:2),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Minus,
        ),
    ],
    token_ranges: [
        [1:1, 1:2),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Binary(
                PureClosed(
                    Mul,
                ),
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:2),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Binary(
                PureClosed(
                    Div,
                ),
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:2),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Binary(
                PureClosed(
                    Power,
                ),
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:3),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Vertical,
        ),
    ],
    token_ranges: [
        [1:1, 1:2),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            DoubleVertical,
        ),
    ],
    token_ranges: [
        [1:1, 1:3),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Binary(
                ShortcuitLogic(
                    And,
                ),
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:3),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Binary(
                Comparison(
                    Eq,
                ),
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:3),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Binary(
                Comparison(
                    Neq,
                ),
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:3),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            RAngle,
        ),
    ],
    token_ranges: [
        [1:1, 1:2),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Binary(
                Comparison(
                    Geq,
                ),
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:3),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            LAngle,
        ),
    ],
    token_ranges: [
        [1:1, 1:2),
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
RangedTokenSheet {
    tokens: [
        Punctuation(
            Binary(
                Comparison(
                    Leq,
                ),
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:3),
    ],
    group_starts: [
        0,
    ],
}
```
