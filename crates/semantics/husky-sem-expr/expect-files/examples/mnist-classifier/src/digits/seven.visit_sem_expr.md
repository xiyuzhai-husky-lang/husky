## `simple_seven_match` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `simple_seven_match` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "simple_leftdown_pattern",
        "[simple_leftdown_pattern]",
        "fermi_match(major_concave_components,[simple_leftdown_pattern])",
        "fermi_match(major_concave_components,[simple_leftdown_pattern])",
        "fermi_match(major_concave_components,[simple_leftdown_pattern])",
    ],
)
```

## `simple_leftdown_pattern` decl

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

## `simple_leftdown_pattern` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y < 0.0",
        "dp.y < 0.0",
        "require dp.y < 0.0",
        "dp",
        "dp.y",
        "-dp.y",
        "-dp.y",
        "let dp = cc.displacement()\n    require dp.y < 0.0\n    -dp.y",
    ],
)
```

## `special_seven_match` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `special_seven_match` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "leftupcc_pattern",
        "leftdowncc_pattern",
        "[leftupcc_pattern, leftdowncc_pattern]",
        "fermi_match(major_concave_components,[leftupcc_pattern, leftdowncc_pattern])",
        "fermi_match(major_concave_components,[leftupcc_pattern, leftdowncc_pattern])",
        "fermi_match(major_concave_components,[leftupcc_pattern, leftdowncc_pattern])",
    ],
)
```

## `leftupcc_pattern` decl

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

## `leftupcc_pattern` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y < 0.0",
        "dp.y < 0.0",
        "require dp.y < 0.0",
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymax()",
        "0.6",
        "cc.relative_bounding_box.ymax() > 0.6",
        "cc.relative_bounding_box.ymax() > 0.6",
        "require cc.relative_bounding_box.ymax() > 0.6",
        "cc",
        "cc.end()",
        "cc.end().y",
        "cc.end().y",
        "let dp = cc.displacement()\n    require dp.y < 0.0\n    require cc.relative_bounding_box.ymax() > 0.6\n    cc.end().y",
    ],
)
```

## `leftdowncc_pattern` decl

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

## `leftdowncc_pattern` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y < 0.0",
        "dp.y < 0.0",
        "require dp.y < 0.0",
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymin()",
        "0.3",
        "cc.relative_bounding_box.ymin() < 0.3",
        "cc.relative_bounding_box.ymin() < 0.3",
        "require cc.relative_bounding_box.ymin() < 0.3",
        "cc",
        "cc.start_tangent()",
        "true",
        "cc.start_tangent().angle(true)",
        "let ang = cc.start_tangent().angle(true)",
        "ang",
        "30.0",
        "ang < 30.0",
        "ang < 30.0",
        "require ang < 30.0",
        "ang",
        "ang",
        "let dp = cc.displacement()\n    require dp.y < 0.0\n\n    require cc.relative_bounding_box.ymin() < 0.3\n\n    let ang = cc.start_tangent().angle(true)\n    require ang < 30.0\n    ang",
    ],
)
```

## `is_seven` decl

```rust
Some(
    [
        "OneVsAll",
        "MnistLabel",
        "OneVsAll MnistLabel",
        "MnistLabel::Seven",
        "OneVsAll MnistLabel MnistLabel::Seven",
    ],
)
```

## `is_seven` defn

```rust
Some(
    [
        "major_connected_component",
        "major_connected_component.max_hole_ilen",
        "0.",
        "major_connected_component.max_hole_ilen == 0.",
        "major_connected_component.max_hole_ilen == 0.",
        "require major_connected_component.max_hole_ilen == 0.",
        "simple_seven_match",
        "simple_seven_match.norm",
        "let simple_match_norm = simple_seven_match.norm",
        "simple_match_norm",
        "1.0",
        "simple_match_norm < 1.0",
        "simple_match_norm < 1.0",
        "simple_seven_match",
        "simple_seven_match.matches",
        "0",
        "simple_seven_match.matches[0]",
        "simple_seven_match.matches[0] be Some(_)",
        "require simple_seven_match.matches[0] be Some(_)",
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component.upper_mass - major_connected_component.lower_mass",
        "let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "upper_excess",
        "10.",
        "upper_excess < 10.",
        "upper_excess < 10.",
        "simple_seven_match",
        "simple_seven_match.matches",
        "0",
        "simple_seven_match.matches[0]",
        "simple_seven_match.matches[0]!",
        "simple_seven_match.matches[0]!.end_tangent()",
        "let end_tangent = simple_seven_match.matches[0]!.end_tangent()",
        "end_tangent",
        "end_tangent.y",
        "let a = end_tangent.y",
        "a",
        "7.0",
        "-7.0",
        "a < -7.0",
        "a < -7.0",
        "require a < -7.0",
        "if upper_excess < 10.:\n            let end_tangent = simple_seven_match.matches[0]!.end_tangent()\n            let a = end_tangent.y\n            require a < -7.0",
        "OneVsAll::Yes",
        "return OneVsAll::Yes",
        "if simple_match_norm < 1.0:\n        require simple_seven_match.matches[0] be Some(_)\n        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        if upper_excess < 10.:\n            let end_tangent = simple_seven_match.matches[0]!.end_tangent()\n            let a = end_tangent.y\n            require a < -7.0\n        return OneVsAll::Yes",
        "simple_match_norm",
        "4.0",
        "simple_match_norm < 4.0",
        "simple_match_norm < 4.0",
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component.upper_mass - major_connected_component.lower_mass",
        "let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "upper_excess",
        "10.",
        "upper_excess > 10.",
        "upper_excess > 10.",
        "require upper_excess > 10.",
        "OneVsAll::Yes",
        "return OneVsAll::Yes",
        "if simple_match_norm < 4.0:\n        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        require upper_excess > 10.\n        return OneVsAll::Yes",
        "special_seven_match",
        "special_seven_match.matches",
        "0",
        "special_seven_match.matches[0]",
        "special_seven_match.matches[0] be Some(_)",
        "require special_seven_match.matches[0] be Some(_)",
        "special_seven_match",
        "special_seven_match.others",
        "let others = special_seven_match.others",
        "false",
        "false",
        "require false",
        "OneVsAll::Yes",
        "OneVsAll::Yes",
        "require major_connected_component.max_hole_ilen == 0.\n    let simple_match_norm = simple_seven_match.norm\n    if simple_match_norm < 1.0:\n        require simple_seven_match.matches[0] be Some(_)\n        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        if upper_excess < 10.:\n            let end_tangent = simple_seven_match.matches[0]!.end_tangent()\n            let a = end_tangent.y\n            require a < -7.0\n        return OneVsAll::Yes\n    if simple_match_norm < 4.0:\n        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        require upper_excess > 10.\n        return OneVsAll::Yes\n    require special_seven_match.matches[0] be Some(_)\n    let others = special_seven_match.others\n    require false\n    OneVsAll::Yes",
    ],
)
```
