## `upper_mouth_match` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `upper_mouth_match` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "big_mouth",
        "[big_mouth]",
        "fermi_match(major_concave_components, [big_mouth])",
        "fermi_match(major_concave_components, [big_mouth])",
        "fermi_match(major_concave_components, [big_mouth])",
    ],
)
```

## `is_eight` decl

```rust
Some(
    [
        "OneVsAll",
        "MnistLabel",
        "OneVsAll MnistLabel",
        "MnistLabel::Eight",
        "OneVsAll MnistLabel MnistLabel::Eight",
    ],
)
```

## `is_eight` defn

```rust
Some(
    [
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component.upper_mass - major_connected_component.lower_mass",
        "let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "major_connected_component",
        "major_connected_component.eff_holes",
        "major_connected_component.eff_holes.matches",
        "1",
        "major_connected_component.eff_holes.matches[1]",
        "major_connected_component.eff_holes.matches[1] be None",
        "major_connected_component",
        "major_connected_component.eff_holes",
        "major_connected_component.eff_holes.matches",
        "0",
        "major_connected_component.eff_holes.matches[0]",
        "major_connected_component.eff_holes.matches[0] be None",
        "false",
        "false",
        "require false",
        "if major_connected_component.eff_holes.matches[0] be None:\n            require false",
        "false",
        "false",
        "require false",
        "if major_connected_component.eff_holes.matches[1] be None:\n        if major_connected_component.eff_holes.matches[0] be None:\n            require false\n        require false",
        "OneVsAll::Yes",
        "OneVsAll::Yes",
        "let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n         \n    // require upper_excess>-5.0 and upper_excess<37.0\n\n    // if major_connected_component.max_hole_ilen == 2.0:\n    //     MnistLabel::Eight\n\n    if major_connected_component.eff_holes.matches[1] be None:\n        if major_connected_component.eff_holes.matches[0] be None:\n            require false\n        require false\n    OneVsAll::Yes",
    ],
)
```

## `big_mouth` decl

```rust
Some(
    [
        "ConcaveComponent",
        "~ConcaveComponent",
        "f32",
        "?f32",
    ],
)
```

## `big_mouth` defn

```rust
Some(
    [
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymax()",
        "0.5",
        "cc.relative_bounding_box.ymax()>0.5",
        "cc.relative_bounding_box.ymax()>0.5",
        "cc",
        "cc.strokes",
        "cc.strokes.first()",
        "cc.strokes.first()!",
        "cc.strokes.first()!.start",
        "cc.strokes.first()!.start.x",
        "cc",
        "cc.strokes",
        "cc.strokes.first()",
        "cc.strokes.first()!",
        "cc.strokes.first()!.end",
        "cc.strokes.first()!.end.x",
        "cc.strokes.first()!.start.x > cc.strokes.first()!.end.x",
        "cc.strokes.first()!.start.x > cc.strokes.first()!.end.x",
        "require cc.strokes.first()!.start.x > cc.strokes.first()!.end.x",
        "if cc.relative_bounding_box.ymax()>0.5:\n        require cc.strokes.first()!.start.x > cc.strokes.first()!.end.x",
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymax()",
        "cc.relative_bounding_box.ymax()",
        "if cc.relative_bounding_box.ymax()>0.5:\n        require cc.strokes.first()!.start.x > cc.strokes.first()!.end.x\n    cc.relative_bounding_box.ymax()",
    ],
)
```
