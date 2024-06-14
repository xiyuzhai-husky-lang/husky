```rust
Some(
    [
        "if major_connected_component.raw_contours.ilen() == 1:\n        let n = open_one_match.norm\n        require n < 1.5\n        require open_one_match.matches[0] be Some(_)\n        require connected_components.ilen() == 1\n        let c = open_one_match.matches[0]!.displacement().norm()\n        require c < 5.5\n        return OneVsAll::Yes",
        "let simp_zero_match = fermi_match(major_concave_components, [])",
        "narrow_down(\n        simp_zero_match.norm,\n        simp_zero_match.rel_norm,\n        simp_zero_match.angle_change_norm,\n        skip = 5\n    )?",
        "require simp_zero_match.norm < 3.0",
        "require major_connected_component.eff_holes.matches[1] be None",
        "require major_connected_component.eff_holes.matches[0] be Some(_)",
        "let major_hole = major_connected_component.eff_holes.matches[0]",
        "let a = major_hole!.bounding_box.ymax() - major_hole!.bounding_box.ymin()",
        "let b = major_line_segment_sketch.bounding_box.ymax() - major_line_segment_sketch.bounding_box.ymin()",
        "let ratio = a/b",
        "require ratio > 0.4",
        "let a = simp_zero_match.norm",
        "OneVsAll::Yes",
        "if major_connected_component.raw_contours.ilen() == 1:\n        let n = open_one_match.norm\n        require n < 1.5\n        require open_one_match.matches[0] be Some(_)\n        require connected_components.ilen() == 1\n        let c = open_one_match.matches[0]!.displacement().norm()\n        require c < 5.5\n        return OneVsAll::Yes\n    let simp_zero_match = fermi_match(major_concave_components, [])\n    narrow_down(\n        simp_zero_match.norm,\n        simp_zero_match.rel_norm,\n        simp_zero_match.angle_change_norm,\n        skip = 5\n    )?\n    require simp_zero_match.norm < 3.0\n    require major_connected_component.eff_holes.matches[1] be None\n    // require major_concave_components.ilen() <= 1 failed\n    require major_connected_component.eff_holes.matches[0] be Some(_)\n    let major_hole = major_connected_component.eff_holes.matches[0]\n    let a = major_hole!.bounding_box.ymax() - major_hole!.bounding_box.ymin()\n    let b = major_line_segment_sketch.bounding_box.ymax() - major_line_segment_sketch.bounding_box.ymin()\n    // high_point, low_point\n    let ratio = a/b\n    require ratio > 0.4\n    let a = simp_zero_match.norm\n    OneVsAll::Yes",
    ],
)
```