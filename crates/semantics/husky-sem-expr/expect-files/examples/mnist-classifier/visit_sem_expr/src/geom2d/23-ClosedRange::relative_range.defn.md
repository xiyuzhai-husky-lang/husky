```rust
Some(
    [
        "assert self.max > self.min",
        "let span = self.max - self.min",
        "let rel_min = (other.min - self.min) / span",
        "let rel_max = (other.max - self.min) / span",
        "ClosedRange(\n            rel_min,\n            rel_max,\n        )",
        "assert self.max > self.min\n        let span = self.max - self.min\n        let rel_min = (other.min - self.min) / span\n        let rel_max = (other.max - self.min) / span\n        ClosedRange(\n            rel_min,\n            rel_max,\n        )",
    ],
)
```