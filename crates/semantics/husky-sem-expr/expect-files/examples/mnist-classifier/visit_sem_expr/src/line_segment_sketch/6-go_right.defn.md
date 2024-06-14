```rust
Some(
    [
        "let L = (u.x*u.x+u.y*u.y).sqrt()",
        "assert L > r",
        "let dr = r*L/(L*L-r*r).sqrt()",
        "let dx = dr*u.y/L",
        "let dy = -dr*u.x/L",
        "Vector2d(u.x+dx, u.y+dy)",
        "let L = (u.x*u.x+u.y*u.y).sqrt()\n    assert L > r\n    let dr = r*L/(L*L-r*r).sqrt()\n    let dx = dr*u.y/L\n    let dy = -dr*u.x/L\n    Vector2d(u.x+dx, u.y+dy)",
    ],
)
```