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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    Closed(
                        Add,
                    ),
                ),
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Minus,
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Star,
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    Closed(
                        Div,
                    ),
                ),
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    Closed(
                        Power,
                    ),
                ),
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Vertical,
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::DoubleVertical,
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    ShortCircuitLogic(
                        And,
                    ),
                ),
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    Comparison(
                        Eq,
                    ),
                ),
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    Comparison(
                        Neq,
                    ),
                ),
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::RaOrGt,
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    Comparison(
                        Geq,
                    ),
                ),
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::LaOrLt,
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    Comparison(
                        Leq,
                    ),
                ),
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
