## `left_components` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `left_components` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "left_coordinate_max",
        "left_coordinate_max",
        "[left_coordinate_max, left_coordinate_max]",
        "fermi_match(major_concave_components, [left_coordinate_max, left_coordinate_max])",
        "fermi_match(major_concave_components, [left_coordinate_max, left_coordinate_max])",
        "fermi_match(major_concave_components, [left_coordinate_max, left_coordinate_max])",
    ],
)
```

## `left_coordinate_max` decl

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

## `left_coordinate_max` defn

```rust
Some(
    [
        "cc",
        "cc.relative_bounding_box",
        "cc.relative_bounding_box.xmax()",
        "cc.relative_bounding_box.xmax()",
        "cc.relative_bounding_box.xmax()",
    ],
)
```

## `components_max_downwards` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `components_max_downwards` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "displacement_downwards",
        "[displacement_downwards]",
        "fermi_match(major_concave_components, [displacement_downwards])",
        "fermi_match(major_concave_components, [displacement_downwards])",
        "fermi_match(major_concave_components, [displacement_downwards])",
    ],
)
```

## `components_max_heights` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `components_max_heights` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "cc_box_heights",
        "[cc_box_heights]",
        "fermi_match(major_concave_components, [cc_box_heights])",
        "fermi_match(major_concave_components, [cc_box_heights])",
        "fermi_match(major_concave_components, [cc_box_heights])",
    ],
)
```

## `is_four` decl

```rust
Some(
    [
        "OneVsAll",
        "MnistLabel",
        "OneVsAll MnistLabel",
        "MnistLabel::Four",
        "OneVsAll MnistLabel MnistLabel::Four",
    ],
)
```

## `is_four` defn

```rust
Some(
    [
        "left_components",
        "left_components.matches",
        "0",
        "left_components.matches[0]",
        "left_components.matches[0] be Some(_)",
        "require left_components.matches[0] be Some(_)",
        "left_components",
        "left_components.matches",
        "1",
        "left_components.matches[1]",
        "left_components.matches[1] be Some(_)",
        "require left_components.matches[1] be Some(_)",
        "major_connected_component",
        "major_connected_component.eff_holes",
        "let eff_holes = major_connected_component.eff_holes",
        "eff_holes",
        "eff_holes.matches",
        "1",
        "eff_holes.matches[1]",
        "eff_holes.matches[1] be None",
        "require eff_holes.matches[1] be None",
        "components_max_downwards",
        "components_max_downwards.matches",
        "0",
        "components_max_downwards.matches[0]",
        "let down_match = components_max_downwards.matches[0]",
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
        "components_max_heights",
        "components_max_heights.matches",
        "0",
        "components_max_heights.matches[0]",
        "let four_match_refine_result = components_max_heights.matches[0]",
        "four_match_refine_result",
        "four_match_refine_result be Some(_)",
        "require four_match_refine_result be Some(_)",
        "components_max_heights",
        "components_max_heights.norm",
        "1.0",
        "components_max_heights.norm <1.0",
        "components_max_heights.norm <1.0",
        "require components_max_heights.norm <1.0",
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component.upper_mass - major_connected_component.lower_mass",
        "let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "components_max_heights",
        "components_max_heights.matches",
        "0",
        "components_max_heights.matches[0]",
        "let upper_arc = components_max_heights.matches[0]",
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
        "components_max_heights",
        "components_max_heights.norm",
        "9.0",
        "components_max_heights.norm < 9.0",
        "components_max_heights.norm < 9.0",
        "require components_max_heights.norm < 9.0",
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
        "if eff_holes.matches[0] be None:\n        require major_concave_components.ilen()>=2\n        let four_match_refine_result = components_max_heights.matches[0]\n        require four_match_refine_result be Some(_)\n        require components_max_heights.norm <1.0\n        let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        let upper_arc = components_max_heights.matches[0]\n        require upper_arc be Some(_)\n        require upper_arc!.displacement().y > 0.0\n        require upper_arc!.angle_change < -110.0\n        require components_max_heights.norm < 9.0\n        let a = major_connected_component.top_k_row_right_mass_sum(3)\n\n        require a < 22.0\n        require a > 9.0\n        return OneVsAll::Yes",
        "OneVsAll::Yes",
        "OneVsAll::Yes",
        "require left_components.matches[0] be Some(_) \n    require left_components.matches[1] be Some(_)\n\n    // left_component_upper = left_components.matches[1]\n    // left_component_lower = left_components.matches[0]\n\n    // if left_components.matches[0].relative_bounding_box.ymax() > left_components.matches[1].relative_bounding_box.ymax():\n    //     left_component_upper = left_components.matches[0]\n    //     left_component_lower = left_components.matches[1]\n    \n    // (a1, a2) =\n    //     if left_components.matches[0].relative_bounding_box.ymax() > left_components.matches[1].relative_bounding_box.ymax():\n    //         left_component_upper = left_components.matches[0]\n    //         left_component_lower = left_components.matches[1]\n    //         (left_component_upper, left_component_lower)\n    //     else:\n    //         (_, _)\n\n\n    // require left_component_upper.strokes.end - left_component_upper.strokes.start == 2\n    // require left_component_lower.matches[1].strokes.end - left_component_lower.strokes.start == 2\n\n\n    \n    // require left_component_upper.\n    \n\n    // angle changes\n\n    // Type II four, with no hole, the horizontal line is short\n\n    let eff_holes = major_connected_component.eff_holes\n\n    require eff_holes.matches[1] be None    // hole.mass\n\n\n\n    // Type III four, with hole, the horizontal line is short \n\n\n\n    // Type IV four, with hole and the horizontal line is short, possibly confused with nine\n\n\n    let down_match = components_max_downwards.matches[0]\n    require down_match be Some(_)\n    let down_match_dp_y = down_match!.displacement().y\n    \n    let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n    require higher_excess>7.0\n\n    if eff_holes.matches[0] be None:\n        require major_concave_components.ilen()>=2\n        let four_match_refine_result = components_max_heights.matches[0]\n        require four_match_refine_result be Some(_)\n        require components_max_heights.norm <1.0\n        let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        let upper_arc = components_max_heights.matches[0]\n        require upper_arc be Some(_)\n        require upper_arc!.displacement().y > 0.0\n        require upper_arc!.angle_change < -110.0\n        require components_max_heights.norm < 9.0\n        let a = major_connected_component.top_k_row_right_mass_sum(3)\n\n        require a < 22.0\n        require a > 9.0\n        return OneVsAll::Yes\n    \n\n    // narrow_down(\n    //     MnistLabel::Four,\n    //     15,\n    //     four_match_refine,\n    //     ignored_connected_components_row_span_sum_sum,\n    //     higher_excess,\n    //     major_connected_component.top_k_row_span_sum(6),\n    // )?\n\n    OneVsAll::Yes",
    ],
)
```

## `displacement_downwards` decl

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

## `displacement_downwards` defn

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

## `cc_box_heights` decl

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

## `cc_box_heights` defn

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
