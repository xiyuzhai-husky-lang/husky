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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Closed(
                        BinaryClosedOpr::Add,
                    ),
                ),
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Minus,
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Star,
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Closed(
                        BinaryClosedOpr::Div,
                    ),
                ),
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Closed(
                        BinaryClosedOpr::Power,
                    ),
                ),
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Vertical,
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::DoubleVertical,
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::ShortCircuitLogic(
                        BinaryShortcuitLogicOpr::And,
                    ),
                ),
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Comparison(
                        BinaryComparisonOpr::Eq,
                    ),
                ),
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Comparison(
                        BinaryComparisonOpr::Neq,
                    ),
                ),
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::RaOrGt,
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Comparison(
                        BinaryComparisonOpr::Geq,
                    ),
                ),
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LaOrLt,
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
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
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Comparison(
                        BinaryComparisonOpr::Leq,
                    ),
                ),
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
    ],
    indents: [
        0,
    ],
}
```
