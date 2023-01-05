# Escapes Gen

## Test#0

input

```husky
"\a"
```

output

```husky
RangedTokenSheet {
    tokens: [
        Err(
            UnexpectedCharAfterBackslash,
        ),
    ],
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    comments: [],
}
```
