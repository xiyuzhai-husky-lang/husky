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
    group_starts: [
        0,
    ],
    indents: [
        0,
    ],
}
```
