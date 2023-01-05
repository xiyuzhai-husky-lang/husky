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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:2),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:2),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:2),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:2),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:3),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:2),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:3),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:3),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:3),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:3),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:2),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:3),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:2),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:3),
    ],
    comments: [],
}
```
