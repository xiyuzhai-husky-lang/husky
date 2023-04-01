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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Minus,
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::Vertical,
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::RaOrGt,
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
        Token::Punctuation(
            Punctuation(
                PunctuationMapped::LaOrLt,
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
    group_starts: [
        0,
    ],
    indents: [
        0,
    ],
}
```
