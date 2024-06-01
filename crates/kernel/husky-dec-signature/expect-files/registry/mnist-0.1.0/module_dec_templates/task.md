```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist::Task`, `TypeAlias`),
            ),
        ),
        Err(
            DecSignatureError::DecTermError,
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist::TASK`, `Static`),
            ),
        ),
        Err(
            DecSignatureError::DecTermError,
        ),
    ),
]
```