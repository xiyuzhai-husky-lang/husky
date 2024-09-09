## `six_match` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `six_match` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "upmost",
        "[upmost]",
        "fermi_match(major_concave_components, [upmost])",
        "fermi_match(major_concave_components, [upmost])",
        "fermi_match(major_concave_components, [upmost])",
    ],
)
```

## `six_match_refined1` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `six_match_refined1` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "upmost",
        "bottom1",
        "[upmost, bottom1]",
        "fermi_match(major_concave_components, [upmost, bottom1])",
        "fermi_match(major_concave_components, [upmost, bottom1])",
        "fermi_match(major_concave_components, [upmost, bottom1])",
    ],
)
```

## `is_six` decl

```rust
Some(
    [
        "OneVsAll",
        "MnistLabel",
        "OneVsAll MnistLabel",
        "MnistLabel::Six",
        "OneVsAll MnistLabel MnistLabel::Six",
    ],
)
```

## `is_six` defn

```rust
Some(
    [
        "six_match",
        "six_match.matches",
        "0",
        "six_match.matches[0]",
        "let upmost_match = six_match.matches[0]",
        "upmost_match",
        "upmost_match be Some(_)",
        "require upmost_match be Some(_)",
        "major_connected_component",
        "major_connected_component.eff_holes",
        "let eff_holes = major_connected_component.eff_holes",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component.lower_mass - major_connected_component.upper_mass",
        "let lower_excess = major_connected_component.lower_mass - major_connected_component.upper_mass",
        "eff_holes",
        "eff_holes.matches",
        "0",
        "eff_holes.matches[0]",
        "eff_holes.matches[0] be None",
        "narrow_down",
        "six_match",
        "six_match.norm",
        "six_match_refined1",
        "six_match_refined1.rel_norm",
        "5",
        "narrow_down(\n            six_match.norm,\n            six_match_refined1.rel_norm,\n            skip=5\n        )",
        "narrow_down(\n            six_match.norm,\n            six_match_refined1.rel_norm,\n            skip=5\n        )?",
        "narrow_down(\n            six_match.norm,\n            six_match_refined1.rel_norm,\n            skip=5\n        )?",
        "six_match_refined1",
        "six_match_refined1.matches",
        "1",
        "six_match_refined1.matches[1]",
        "let bottom1_match = six_match_refined1.matches[1]",
        "bottom1_match",
        "bottom1_match!",
        "bottom1_match!.displacement()",
        "let bottom1_match_dp = bottom1_match!.displacement()",
        "bottom1_match_dp",
        "bottom1_match_dp.y",
        "let bottom1_match_dp_y = bottom1_match_dp.y",
        "upmost_match",
        "upmost_match!",
        "upmost_match!.displacement()",
        "upmost_match!.displacement().y",
        "let upmost_match_dp_y = upmost_match!.displacement().y",
        "six_match_refined1",
        "six_match_refined1.others",
        "let others = six_match_refined1.others",
        "six_match_refined1",
        "six_match_refined1.norm",
        "1.8",
        "six_match_refined1.norm < 1.8",
        "six_match_refined1.norm < 1.8",
        "require six_match_refined1.norm < 1.8",
        "bottom1_match",
        "bottom1_match be Some(_)",
        "bottom1_match_dp_y",
        "2.5",
        "-2.5",
        "bottom1_match_dp_y > -2.5",
        "bottom1_match_dp_y > -2.5",
        "require bottom1_match_dp_y > -2.5",
        "ignored_connected_components_row_span_sum_sum",
        "30.0",
        "ignored_connected_components_row_span_sum_sum < 30.0",
        "ignored_connected_components_row_span_sum_sum < 30.0",
        "require ignored_connected_components_row_span_sum_sum < 30.0",
        "narrow_down",
        "upmost_match_dp_y",
        "15",
        "narrow_down(\n                upmost_match_dp_y,\n                skip = 15\n            )",
        "narrow_down(\n                upmost_match_dp_y,\n                skip = 15\n            )?",
        "narrow_down(\n                upmost_match_dp_y,\n                skip = 15\n            )?",
        "OneVsAll::Yes",
        "return OneVsAll::Yes",
        "if bottom1_match be Some(_):\n            require bottom1_match_dp_y > -2.5\n            // ad hoc\n            require ignored_connected_components_row_span_sum_sum < 30.0\n            narrow_down(\n                upmost_match_dp_y,\n                skip = 15\n            )?\n            return OneVsAll::Yes",
        "major_line_segment_sketch",
        "major_line_segment_sketch.bounding_box",
        "upmost_match",
        "upmost_match!",
        "upmost_match!.end()",
        "major_line_segment_sketch.bounding_box.relative_point(\n            upmost_match!.end()\n        )",
        "let rel_upmost_match_end = major_line_segment_sketch.bounding_box.relative_point(\n            upmost_match!.end()\n        )",
        "narrow_down",
        "upmost_match_dp_y",
        "ignored_connected_components_row_span_sum_sum",
        "lower_excess",
        "major_connected_component",
        "6",
        "major_connected_component.top_k_row_span_sum(6)",
        "15",
        "narrow_down(\n            upmost_match_dp_y,\n            ignored_connected_components_row_span_sum_sum,\n            lower_excess,\n            major_connected_component.top_k_row_span_sum(6),\n            skip = 15\n        )",
        "narrow_down(\n            upmost_match_dp_y,\n            ignored_connected_components_row_span_sum_sum,\n            lower_excess,\n            major_connected_component.top_k_row_span_sum(6),\n            skip = 15\n        )?",
        "narrow_down(\n            upmost_match_dp_y,\n            ignored_connected_components_row_span_sum_sum,\n            lower_excess,\n            major_connected_component.top_k_row_span_sum(6),\n            skip = 15\n        )?",
        "rel_upmost_match_end",
        "rel_upmost_match_end.x",
        "0.7",
        "rel_upmost_match_end.x < 0.7",
        "rel_upmost_match_end.x < 0.7",
        "OneVsAll::Yes",
        "return OneVsAll::Yes",
        "if rel_upmost_match_end.x < 0.7:\n            return OneVsAll::Yes",
        "OneVsAll::Yes",
        "return OneVsAll::Yes",
        "if eff_holes.matches[0] be None:\n        narrow_down(\n            six_match.norm,\n            six_match_refined1.rel_norm,\n            skip=5\n        )?\n        let bottom1_match = six_match_refined1.matches[1]\n        let bottom1_match_dp = bottom1_match!.displacement()\n        let bottom1_match_dp_y = bottom1_match_dp.y\n        let upmost_match_dp_y = upmost_match!.displacement().y\n        let others = six_match_refined1.others\n        require six_match_refined1.norm < 1.8\n        if bottom1_match be Some(_):\n            require bottom1_match_dp_y > -2.5\n            // ad hoc\n            require ignored_connected_components_row_span_sum_sum < 30.0\n            narrow_down(\n                upmost_match_dp_y,\n                skip = 15\n            )?\n            return OneVsAll::Yes\n        let rel_upmost_match_end = major_line_segment_sketch.bounding_box.relative_point(\n            upmost_match!.end()\n        )\n        narrow_down(\n            upmost_match_dp_y,\n            ignored_connected_components_row_span_sum_sum,\n            lower_excess,\n            major_connected_component.top_k_row_span_sum(6),\n            skip = 15\n        )?\n        if rel_upmost_match_end.x < 0.7:\n            return OneVsAll::Yes\n        return OneVsAll::Yes",
        "narrow_down",
        "six_match",
        "six_match.norm",
        "5",
        "narrow_down(\n        six_match.norm,\n        skip = 5\n    )",
        "narrow_down(\n        six_match.norm,\n        skip = 5\n    )?",
        "narrow_down(\n        six_match.norm,\n        skip = 5\n    )?",
        "six_match",
        "six_match.norm",
        "1.8",
        "six_match.norm > 1.8",
        "six_match.norm > 1.8",
        "six_match",
        "six_match.norm",
        "1.8",
        "six_match.norm < 1.8",
        "six_match.norm < 1.8",
        "require six_match.norm < 1.8",
        "if six_match.norm > 1.8:\n        require six_match.norm < 1.8",
        "eff_holes",
        "eff_holes.matches",
        "0",
        "eff_holes.matches[0]",
        "eff_holes.matches[0]!",
        "eff_holes.matches[0]!.relative_bounding_box",
        "eff_holes.matches[0]!.relative_bounding_box.ymax()",
        "0.75",
        "eff_holes.matches[0]!.relative_bounding_box.ymax() < 0.75",
        "eff_holes.matches[0]!.relative_bounding_box.ymax() < 0.75",
        "require eff_holes.matches[0]!.relative_bounding_box.ymax() < 0.75",
        "eff_holes",
        "eff_holes.matches",
        "1",
        "eff_holes.matches[1]",
        "eff_holes.matches[1] be Some(_)",
        "eff_holes",
        "eff_holes.matches",
        "1",
        "eff_holes.matches[1]",
        "eff_holes.matches[1]!",
        "eff_holes.matches[1]!.relative_bounding_box",
        "eff_holes.matches[1]!.relative_bounding_box.ymax()",
        "0.75",
        "eff_holes.matches[1]!.relative_bounding_box.ymax() < 0.75",
        "eff_holes.matches[1]!.relative_bounding_box.ymax() < 0.75",
        "require eff_holes.matches[1]!.relative_bounding_box.ymax() < 0.75",
        "if eff_holes.matches[1] be Some(_):\n        require eff_holes.matches[1]!.relative_bounding_box.ymax() < 0.75",
        "lower_excess",
        "15.0",
        "lower_excess > 15.0",
        "lower_excess > 15.0",
        "require lower_excess > 15.0",
        "OneVsAll::Yes",
        "OneVsAll::Yes",
        "let upmost_match = six_match.matches[0]\n    require upmost_match be Some(_)\n    let eff_holes = major_connected_component.eff_holes\n    let lower_excess = major_connected_component.lower_mass - major_connected_component.upper_mass\n    if eff_holes.matches[0] be None:\n        narrow_down(\n            six_match.norm,\n            six_match_refined1.rel_norm,\n            skip=5\n        )?\n        let bottom1_match = six_match_refined1.matches[1]\n        let bottom1_match_dp = bottom1_match!.displacement()\n        let bottom1_match_dp_y = bottom1_match_dp.y\n        let upmost_match_dp_y = upmost_match!.displacement().y\n        let others = six_match_refined1.others\n        require six_match_refined1.norm < 1.8\n        if bottom1_match be Some(_):\n            require bottom1_match_dp_y > -2.5\n            // ad hoc\n            require ignored_connected_components_row_span_sum_sum < 30.0\n            narrow_down(\n                upmost_match_dp_y,\n                skip = 15\n            )?\n            return OneVsAll::Yes\n        let rel_upmost_match_end = major_line_segment_sketch.bounding_box.relative_point(\n            upmost_match!.end()\n        )\n        narrow_down(\n            upmost_match_dp_y,\n            ignored_connected_components_row_span_sum_sum,\n            lower_excess,\n            major_connected_component.top_k_row_span_sum(6),\n            skip = 15\n        )?\n        if rel_upmost_match_end.x < 0.7:\n            return OneVsAll::Yes\n        return OneVsAll::Yes\n    // narrow_down: filter the data that satisfy the given features. \n    narrow_down(\n        six_match.norm,\n        skip = 5\n    )?\n    if six_match.norm > 1.8:\n        require six_match.norm < 1.8\n    require eff_holes.matches[0]!.relative_bounding_box.ymax() < 0.75\n    if eff_holes.matches[1] be Some(_):\n        require eff_holes.matches[1]!.relative_bounding_box.ymax() < 0.75\n    require lower_excess > 15.0\n    OneVsAll::Yes",
    ],
)
```

## `upmost` decl

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

## `upmost` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y > 0.0",
        "dp.y > 0.0",
        "require dp.y > 0.0",
        "dp",
        "dp.y",
        "dp.y",
        "let dp = cc.displacement()\n    require dp.y > 0.0\n    dp.y",
    ],
)
```

## `bottom1` decl

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

## `bottom1` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "3.0",
        "-3.0",
        "dp.y < -3.0",
        "dp.y < -3.0",
        "dp",
        "dp.x",
        "dp",
        "dp.y",
        "dp.x / dp.y",
        "(dp.x / dp.y)",
        "(dp.x / dp.y).abs()",
        "1.4",
        "(dp.x / dp.y).abs() > 1.4",
        "(dp.x / dp.y).abs() > 1.4",
        "require (dp.x / dp.y).abs() > 1.4",
        "if dp.y < -3.0:\n        require (dp.x / dp.y).abs() > 1.4",
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymax()",
        "0.6",
        "cc.relative_bounding_box.ymax() < 0.6",
        "cc.relative_bounding_box.ymax() < 0.6",
        "require cc.relative_bounding_box.ymax() < 0.6",
        "cc",
        "cc.line_segment_sketch",
        "cc.line_segment_sketch.bounding_box",
        "cc",
        "cc.end()",
        "cc.line_segment_sketch.bounding_box.relative_point(cc.end())",
        "let relative_end = cc.line_segment_sketch.bounding_box.relative_point(cc.end())",
        "relative_end",
        "relative_end.x",
        "0.5",
        "relative_end.x > 0.5",
        "relative_end.x > 0.5",
        "require relative_end.x > 0.5",
        "cc",
        "cc.end()",
        "cc.end().y",
        "-cc.end().y",
        "-cc.end().y",
        "let dp = cc.displacement()\n    if dp.y < -3.0:\n        require (dp.x / dp.y).abs() > 1.4\n    require cc.relative_bounding_box.ymax() < 0.6\n    let relative_end = cc.line_segment_sketch.bounding_box.relative_point(cc.end())\n    require relative_end.x > 0.5\n    -cc.end().y",
    ],
)
```
