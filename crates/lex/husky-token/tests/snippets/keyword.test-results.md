# Keyword

## Test#0

input

```husky
def
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Keyword(
            Keyword::Form(
                Def,
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
func
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Ident(
            `func`,
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
proc
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Ident(
            `proc`,
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
trait
```

output

```husky
TokenSheetData {
    tokens: [
        Token::Keyword(
            Keyword::Trait,
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
