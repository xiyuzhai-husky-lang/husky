# Binary Opr

## Test#0

input

```husky
+
```

output

```husky
TokenSheetData {
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
    indents: [
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
TokenSheetData {
    tokens: [
        Punctuation(
            Minus,
        ),
    ],
    group_starts: [
        0,
    ],
    indents: [
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
TokenSheetData {
    tokens: [
        Punctuation(
            Star,
        ),
    ],
    group_starts: [
        0,
    ],
    indents: [
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
TokenSheetData {
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
    indents: [
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
TokenSheetData {
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
    indents: [
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
TokenSheetData {
    tokens: [
        Punctuation(
            Vertical,
        ),
    ],
    group_starts: [
        0,
    ],
    indents: [
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
TokenSheetData {
    tokens: [
        Punctuation(
            DoubleVertical,
        ),
    ],
    group_starts: [
        0,
    ],
    indents: [
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
TokenSheetData {
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
    indents: [
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
TokenSheetData {
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
    indents: [
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
TokenSheetData {
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
    indents: [
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
TokenSheetData {
    tokens: [
        Punctuation(
            RaOrGt,
        ),
    ],
    group_starts: [
        0,
    ],
    indents: [
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
TokenSheetData {
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
    indents: [
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
TokenSheetData {
    tokens: [
        Punctuation(
            LaOrLt,
        ),
    ],
    group_starts: [
        0,
    ],
    indents: [
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
TokenSheetData {
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
    indents: [
        0,
    ],
}
```
