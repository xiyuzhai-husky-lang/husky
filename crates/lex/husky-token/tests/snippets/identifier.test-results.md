# Identifier

## Test#0

input

```husky
self
```

output

```husky
TokenSheetData {
    tokens: [
        Keyword(
            Pronoun(
                SelfValue,
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
Self
```

output

```husky
TokenSheetData {
    tokens: [
        Keyword(
            Pronoun(
                SelfType,
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
