```rust
Some(
    [
        "let eff_holes = major_connected_component.eff_holes",
        "require eff_holes.matches[1] be None",
        "let down_match=nine_match.matches[0]",
        "require down_match be Some(_)",
        "let down_match_dp_y = down_match!.displacement().y",
        "let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "require higher_excess>7.0",
        "if eff_holes.matches[0] be None:\n        require major_concave_components.ilen()>=2\n        let nine_match_refine_result=nine_match_refine.matches[0]\n        require nine_match_refine_result be Some(_)\n        require nine_match_refine.norm <1.0\n        let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        let upper_arc = nine_match_refine.matches[0]\n        require upper_arc be Some(_)\n        require upper_arc!.displacement().y > 0.0\n        require upper_arc!.angle_change < -110.0\n        require nine_match_refine.norm < 9.0\n        let a = major_connected_component.top_k_row_right_mass_sum(3)\n\n        require a < 22.0\n        require a > 9.0\n        return OneVsAll::Yes",
        "OneVsAll::Yes",
        "let eff_holes = major_connected_component.eff_holes\n\n    require eff_holes.matches[1] be None\n\n    let down_match=nine_match.matches[0]\n    require down_match be Some(_)\n    let down_match_dp_y = down_match!.displacement().y\n    \n    let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n    require higher_excess>7.0\n\n    if eff_holes.matches[0] be None:\n        require major_concave_components.ilen()>=2\n        let nine_match_refine_result=nine_match_refine.matches[0]\n        require nine_match_refine_result be Some(_)\n        require nine_match_refine.norm <1.0\n        let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        let upper_arc = nine_match_refine.matches[0]\n        require upper_arc be Some(_)\n        require upper_arc!.displacement().y > 0.0\n        require upper_arc!.angle_change < -110.0\n        require nine_match_refine.norm < 9.0\n        let a = major_connected_component.top_k_row_right_mass_sum(3)\n\n        require a < 22.0\n        require a > 9.0\n        return OneVsAll::Yes\n    \n\n    // narrow_down(\n    //     MnistLabel::Nine,\n    //     15,\n    //     nine_match_refine,\n    //     ignored_connected_components_row_span_sum_sum,\n    //     higher_excess,\n    //     major_connected_component.top_k_row_span_sum(6),\n    // )?\n\n    OneVsAll::Yes",
    ],
)
```