## `three_fermi_match` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `three_fermi_match` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "downarc",
        "uparc",
        "back",
        "[downarc, uparc, back]",
        "fermi_match(major_concave_components, [downarc, uparc, back])",
        "fermi_match(major_concave_components, [downarc, uparc, back])",
        "fermi_match(major_concave_components, [downarc, uparc, back])",
    ],
)
```

## `is_three` decl

```rust
Some(
    [
        "OneVsAll",
        "MnistLabel",
        "OneVsAll MnistLabel",
        "MnistLabel::Three",
        "OneVsAll MnistLabel MnistLabel::Three",
    ],
)
```

## `is_three` defn

```rust
Some(
    [
        "major_concave_components",
        "major_concave_components.ilen()",
        "2",
        "major_concave_components.ilen() >= 2",
        "major_concave_components.ilen() >= 2",
        "require major_concave_components.ilen() >= 2",
        "major_concave_components",
        "major_concave_components.ilen()",
        "4",
        "major_concave_components.ilen() <= 4",
        "major_concave_components.ilen() <= 4",
        "require major_concave_components.ilen() <= 4",
        "three_fermi_match",
        "three_fermi_match.matches",
        "0",
        "three_fermi_match.matches[0]",
        "let downarc = three_fermi_match.matches[0]",
        "three_fermi_match",
        "three_fermi_match.matches",
        "1",
        "three_fermi_match.matches[1]",
        "let uparc = three_fermi_match.matches[1]",
        "three_fermi_match",
        "three_fermi_match.matches",
        "2",
        "three_fermi_match.matches[2]",
        "let back = three_fermi_match.matches[2]",
        "downarc",
        "downarc be Some(_)",
        "require downarc be Some(_)",
        "downarc",
        "downarc!",
        "downarc!.norm",
        "3.0",
        "downarc!.norm > 3.0",
        "downarc!.norm > 3.0",
        "require downarc!.norm > 3.0",
        "uparc",
        "uparc be Some(_)",
        "require uparc be Some(_)",
        "downarc",
        "downarc!",
        "downarc!.end_tangent()",
        "true",
        "downarc!.end_tangent().angle(true)",
        "let de = downarc!.end_tangent().angle(true)",
        "de",
        "0.0",
        "de > 0.0",
        "de",
        "100.0",
        "-100.0",
        "de < -100.0",
        "de > 0.0 or de < -100.0",
        "de > 0.0 or de < -100.0",
        "require de > 0.0 or de < -100.0",
        "downarc",
        "downarc!",
        "downarc!.end()",
        "let downarc_enpoint = downarc!.end()",
        "uparc",
        "uparc!",
        "uparc!.start()",
        "let uparc_startpoint = uparc!.start()",
        "downarc_enpoint",
        "uparc_startpoint",
        "downarc_enpoint.dist(uparc_startpoint)",
        "let distance = downarc_enpoint.dist(uparc_startpoint)",
        "distance",
        "20.0",
        "distance < 20.0",
        "distance < 20.0",
        "require distance < 20.0",
        "three_fermi_match",
        "three_fermi_match.norm",
        "2.5",
        "three_fermi_match.norm < 2.5",
        "three_fermi_match.norm < 2.5",
        "require three_fermi_match.norm < 2.5",
        "downarc",
        "downarc!",
        "downarc!.angle_change",
        "100.0",
        "-100.0",
        "downarc!.angle_change < -100.0",
        "downarc!.angle_change < -100.0",
        "require downarc!.angle_change < -100.0",
        "OneVsAll::Yes",
        "OneVsAll::Yes",
        "require major_concave_components.ilen() >= 2\n    require major_concave_components.ilen() <= 4\n    let downarc = three_fermi_match.matches[0]\n    let uparc = three_fermi_match.matches[1]\n    let back = three_fermi_match.matches[2]\n    require downarc be Some(_)    \n    require downarc!.norm > 3.0\n    require uparc be Some(_)\n\n    let de = downarc!.end_tangent().angle(true)\n    require de > 0.0 or de < -100.0\n    let downarc_enpoint = downarc!.end()\n    let uparc_startpoint = uparc!.start()\n    let distance = downarc_enpoint.dist(uparc_startpoint)\n    require distance < 20.0\n\n    // to improve\n    require three_fermi_match.norm < 2.5\n    require downarc!.angle_change < -100.0\n\n    OneVsAll::Yes",
    ],
)
```

## `uparc` decl

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

## `uparc` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y <= 0.0",
        "dp.y <= 0.0",
        "require dp.y <= 0.0",
        "Some",
        "cc",
        "cc.bounding_box",
        "cc.bounding_box.ymin()",
        "-cc.bounding_box.ymin()",
        "Some(-cc.bounding_box.ymin())",
        "Some(-cc.bounding_box.ymin())",
        "let dp = cc.displacement()\n    require dp.y <= 0.0\n    Some(-cc.bounding_box.ymin())",
    ],
)
```

## `downarc` decl

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

## `downarc` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y <= 0.0",
        "dp.y <= 0.0",
        "require dp.y <= 0.0",
        "Some",
        "cc",
        "cc.bounding_box",
        "cc.bounding_box.ymin()",
        "-cc.bounding_box.ymin()",
        "Some(-cc.bounding_box.ymin())",
        "Some(-cc.bounding_box.ymin())",
        "let dp = cc.displacement()\n    require dp.y <= 0.0\n    Some(-cc.bounding_box.ymin())",
    ],
)
```

## `back` decl

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

## `back` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y >= 0.0",
        "dp.y >= 0.0",
        "require dp.y >= 0.0",
        "Some",
        "cc",
        "cc.bounding_box",
        "cc.bounding_box.ymin()",
        "-cc.bounding_box.ymin()",
        "Some(-cc.bounding_box.ymin())",
        "Some(-cc.bounding_box.ymin())",
        "let dp = cc.displacement()\n    require dp.y >= 0.0\n    Some(-cc.bounding_box.ymin())",
    ],
)
```
