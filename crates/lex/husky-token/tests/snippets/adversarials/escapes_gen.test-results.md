# Escapes Gen

## Test#0

input

```husky
"\a"
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:5),
            kind: Err(
                UnexpectedCharAfterBackslash,
            ),
        },
    ],
    group_starts: [
        0,
    ],
}
```
