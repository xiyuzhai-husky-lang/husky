```rust
Some(
    [
        "self",
        "self.x",
        "self",
        "self.x",
        "self.x * self.x",
        "self",
        "self.y",
        "self",
        "self.y",
        "self.y * self.y",
        "self.x * self.x + self.y * self.y",
        "(self.x * self.x + self.y * self.y)",
        "(self.x * self.x + self.y * self.y).sqrt()",
        "(self.x * self.x + self.y * self.y).sqrt()",
        "(self.x * self.x + self.y * self.y).sqrt()",
    ],
)
```