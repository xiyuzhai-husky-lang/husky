# Literal

## Test#0

input

```husky
"haha"
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Literal(
            Literal::String(
                StringLiteral {
                    data: "haha",
                },
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupBase(
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
"\n"
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Literal(
            Literal::String(
                StringLiteral {
                    data: "\n",
                },
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupBase(
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
"\t"
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Literal(
            Literal::String(
                StringLiteral {
                    data: "\t",
                },
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupBase(
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
"\\"
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Literal(
            Literal::String(
                StringLiteral {
                    data: "\\",
                },
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupBase(
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
"\""
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Literal(
            Literal::String(
                StringLiteral {
                    data: "\"",
                },
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupBase(
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
"\r"
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Literal(
            Literal::String(
                StringLiteral {
                    data: "\r",
                },
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupBase(
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
