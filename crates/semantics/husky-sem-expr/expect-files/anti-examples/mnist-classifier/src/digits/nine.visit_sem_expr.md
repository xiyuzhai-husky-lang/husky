## `nine_match` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `nine_match` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "downmost",
        "[downmost]",
        "fermi_match(major_concave_components, [downmost])",
        "fermi_match(major_concave_components, [downmost])",
        "fermi_match(major_concave_components, [downmost])",
    ],
)
```

## `nine_match_refine` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `nine_match_refine` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "big_cc",
        "[big_cc]",
        "fermi_match(major_concave_components, [big_cc])",
        "fermi_match(major_concave_components, [big_cc])",
        "fermi_match(major_concave_components, [big_cc])",
    ],
)
```

## `is_nine` decl

```rust
Some(
    [
        "OneVsAll",
        "MnistLabel",
        "OneVsAll MnistLabel",
        "MnistLabel::Nine",
        "OneVsAll MnistLabel MnistLabel::Nine",
    ],
)
```

## `is_nine` defn

```rust
Some(
    [
        "major_connected_component",
        "major_connected_component.eff_holes",
        "let eff_holes = major_connected_component.eff_holes",
        "eff_holes",
        "eff_holes.matches",
        "1",
        "eff_holes.matches[1]",
        "eff_holes.matches[1] be None",
        "require eff_holes.matches[1] be None",
        "nine_match",
        "nine_match.matches",
        "0",
        "nine_match.matches[0]",
        "let down_match=nine_match.matches[0]",
        "down_match",
        "down_match be Some(_)",
        "require down_match be Some(_)",
        "down_match",
        "down_match!",
        "down_match!.displacement()",
        "down_match!.displacement().y",
        "let down_match_dp_y = down_match!.displacement().y",
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component.upper_mass - major_connected_component.lower_mass",
        "let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "higher_excess",
        "7.0",
        "higher_excess>7.0",
        "higher_excess>7.0",
        "require higher_excess>7.0",
        "eff_holes",
        "eff_holes.matches",
        "0",
        "eff_holes.matches[0]",
        "eff_holes.matches[0] be None",
        "major_concave_components",
        "major_concave_components.ilen()",
        "2",
        "major_concave_components.ilen()>=2",
        "major_concave_components.ilen()>=2",
        "require major_concave_components.ilen()>=2",
        "nine_match_refine",
        "nine_match_refine.matches",
        "0",
        "nine_match_refine.matches[0]",
        "let nine_match_refine_result=nine_match_refine.matches[0]",
        "nine_match_refine_result",
        "nine_match_refine_result be Some(_)",
        "require nine_match_refine_result be Some(_)",
        "nine_match_refine",
        "nine_match_refine.norm",
        "1.0",
        "nine_match_refine.norm <1.0",
        "nine_match_refine.norm <1.0",
        "require nine_match_refine.norm <1.0",
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component.upper_mass - major_connected_component.lower_mass",
        "let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "nine_match_refine",
        "nine_match_refine.matches",
        "0",
        "nine_match_refine.matches[0]",
        "let upper_arc = nine_match_refine.matches[0]",
        "upper_arc",
        "upper_arc be Some(_)",
        "require upper_arc be Some(_)",
        "upper_arc",
        "upper_arc!",
        "upper_arc!.displacement()",
        "upper_arc!.displacement().y",
        "0.0",
        "upper_arc!.displacement().y > 0.0",
        "upper_arc!.displacement().y > 0.0",
        "require upper_arc!.displacement().y > 0.0",
        "upper_arc",
        "upper_arc!",
        "upper_arc!.angle_change",
        "110.0",
        "-110.0",
        "upper_arc!.angle_change < -110.0",
        "upper_arc!.angle_change < -110.0",
        "require upper_arc!.angle_change < -110.0",
        "nine_match_refine",
        "nine_match_refine.norm",
        "9.0",
        "nine_match_refine.norm < 9.0",
        "nine_match_refine.norm < 9.0",
        "require nine_match_refine.norm < 9.0",
        "major_connected_component",
        "3",
        "major_connected_component.top_k_row_right_mass_sum(3)",
        "let a = major_connected_component.top_k_row_right_mass_sum(3)",
        "a",
        "22.0",
        "a < 22.0",
        "a < 22.0",
        "require a < 22.0",
        "a",
        "9.0",
        "a > 9.0",
        "a > 9.0",
        "require a > 9.0",
        "OneVsAll::Yes",
        "return OneVsAll::Yes",
        "if eff_holes.matches[0] be None:\n        require major_concave_components.ilen()>=2\n        let nine_match_refine_result=nine_match_refine.matches[0]\n        require nine_match_refine_result be Some(_)\n        require nine_match_refine.norm <1.0\n        let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        let upper_arc = nine_match_refine.matches[0]\n        require upper_arc be Some(_)\n        require upper_arc!.displacement().y > 0.0\n        require upper_arc!.angle_change < -110.0\n        require nine_match_refine.norm < 9.0\n        let a = major_connected_component.top_k_row_right_mass_sum(3)\n\n        require a < 22.0\n        require a > 9.0\n        return OneVsAll::Yes",
        "OneVsAll::Yes",
        "OneVsAll::Yes",
        "let eff_holes = major_connected_component.eff_holes\n\n    require eff_holes.matches[1] be None\n\n    let down_match=nine_match.matches[0]\n    require down_match be Some(_)\n    let down_match_dp_y = down_match!.displacement().y\n    \n    let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n    require higher_excess>7.0\n\n    if eff_holes.matches[0] be None:\n        require major_concave_components.ilen()>=2\n        let nine_match_refine_result=nine_match_refine.matches[0]\n        require nine_match_refine_result be Some(_)\n        require nine_match_refine.norm <1.0\n        let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        let upper_arc = nine_match_refine.matches[0]\n        require upper_arc be Some(_)\n        require upper_arc!.displacement().y > 0.0\n        require upper_arc!.angle_change < -110.0\n        require nine_match_refine.norm < 9.0\n        let a = major_connected_component.top_k_row_right_mass_sum(3)\n\n        require a < 22.0\n        require a > 9.0\n        return OneVsAll::Yes\n    \n\n    // narrow_down(\n    //     MnistLabel::Nine,\n    //     15,\n    //     nine_match_refine,\n    //     ignored_connected_components_row_span_sum_sum,\n    //     higher_excess,\n    //     major_connected_component.top_k_row_span_sum(6),\n    // )?\n\n    OneVsAll::Yes",
    ],
)
```

## `downmost` decl

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

## `downmost` defn

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
        "dp.y",
        "let dp = cc.displacement()\n    require dp.y < 0.0\n    dp.y",
    ],
)
```

## `big_cc` decl

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

## `big_cc` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y >0.0",
        "dp.y >0.0",
        "require dp.y >0.0",
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymin()",
        "0.4",
        "cc.relative_bounding_box.ymin()>0.4",
        "cc.relative_bounding_box.ymin()>0.4",
        "require cc.relative_bounding_box.ymin()>0.4",
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.ymin()",
        "cc.relative_bounding_box.ymin()",
        "let dp = cc.displacement()\n    require dp.y >0.0\n    require cc.relative_bounding_box.ymin()>0.4\n    // dp = cc.displacement()\n    // require dp.y.abs()<7.0\n    cc.relative_bounding_box.ymin()",
    ],
)
```
