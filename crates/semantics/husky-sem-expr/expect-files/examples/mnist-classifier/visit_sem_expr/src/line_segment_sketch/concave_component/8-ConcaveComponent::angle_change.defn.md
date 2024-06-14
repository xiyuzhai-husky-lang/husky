```rust
Some(
    [
        "let mut angle_change = 0.0",
        "let mut dp0 = self.strokes[self.strokes.start()].displacement()",
        "for self.strokes.start() < i < self.strokes.end():\n            let dp = self.strokes[i].displacement()\n            angle_change += dp0.angle_to(dp, true)\n            dp0 = dp",
        "return angle_change",
        "let mut angle_change = 0.0\n        let mut dp0 = self.strokes[self.strokes.start()].displacement()\n        // todo: change self for .. in ..[1..]\n        for self.strokes.start() < i < self.strokes.end():\n            let dp = self.strokes[i].displacement()\n            angle_change += dp0.angle_to(dp, true)\n            dp0 = dp\n        return angle_change",
    ],
)
```