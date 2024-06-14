```rust
Some(
    [
        "let cc_num = major_concave_components.ilen()",
        "let eff_holes = major_connected_component.eff_holes",
        "require eff_holes.matches[1] be None",
        "let left_cc = two_match.matches[0]",
        "let right_cc = two_match.matches[1]",
        "let down_cc = two_match.matches[2]",
        "require cc_num<=3",
        "let lower_excess = major_connected_component.lower_mass - major_connected_component.upper_mass",
        "require lower_excess > 10.0",
        "if cc_num == 2:\n        require left_cc be Some(_)\n        require right_cc be Some(_)\n        let a = right_cc!.angle_change\n        require a>-180.0\n        // get the end line of the cc\n        let end_tan = left_cc!.end_tangent().angle(true)\n        // require end_tan < -200.0\n        let x = left_cc!.end_tangent().x\n        let y = left_cc!.end_tangent().y\n\n        let left_ymax = left_cc!.relative_bounding_box.ymax()\n        let left_ymin = left_cc!.relative_bounding_box.ymin()\n        let left_mid_y = (left_ymax + left_ymin) / 2.0\n\n        let right_ymax = right_cc!.relative_bounding_box.ymax()\n        let right_ymin = right_cc!.relative_bounding_box.ymin()\n        let right_mid_y = (right_ymax + right_ymin) / 2.0\n        require left_mid_y >= right_mid_y",
        "if cc_num==3:\n        require left_cc be Some(_)\n        require right_cc be Some(_)\n        require down_cc be Some(_)\n        require down_cc!.relative_bounding_box.ymin() <0.4\n        let a = down_cc!.angle_change",
        "OneVsAll::Yes",
        "let cc_num = major_concave_components.ilen()\n    let eff_holes = major_connected_component.eff_holes\n    \n    require eff_holes.matches[1] be None\n    let left_cc = two_match.matches[0]\n    let right_cc = two_match.matches[1]\n    let down_cc = two_match.matches[2]\n\n    require cc_num<=3\n\n    let lower_excess = major_connected_component.lower_mass - major_connected_component.upper_mass\n\n    require lower_excess > 10.0\n\n    if cc_num == 2:\n        require left_cc be Some(_)\n        require right_cc be Some(_)\n        let a = right_cc!.angle_change\n        require a>-180.0\n        // get the end line of the cc\n        let end_tan = left_cc!.end_tangent().angle(true)\n        // require end_tan < -200.0\n        let x = left_cc!.end_tangent().x\n        let y = left_cc!.end_tangent().y\n\n        let left_ymax = left_cc!.relative_bounding_box.ymax()\n        let left_ymin = left_cc!.relative_bounding_box.ymin()\n        let left_mid_y = (left_ymax + left_ymin) / 2.0\n\n        let right_ymax = right_cc!.relative_bounding_box.ymax()\n        let right_ymin = right_cc!.relative_bounding_box.ymin()\n        let right_mid_y = (right_ymax + right_ymin) / 2.0\n        require left_mid_y >= right_mid_y\n\n    // if cc_num == 2:\n    //     require major_connected_component.eff_holes.matches[0] be None\n    if cc_num==3:\n        require left_cc be Some(_)\n        require right_cc be Some(_)\n        require down_cc be Some(_)\n        require down_cc!.relative_bounding_box.ymin() <0.4\n        let a = down_cc!.angle_change\n    OneVsAll::Yes",
    ],
)
```