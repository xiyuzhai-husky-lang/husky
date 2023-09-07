# Escapes Gen

## Test#0

input

```husky
"\a"
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Error(
            TokenError::UnexpectedCharAfterBackslash(
                'a',
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
