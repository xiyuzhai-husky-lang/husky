```rust
Some(
    [
        "require major_concave_components.ilen() >= 2",
        "require major_concave_components.ilen() <= 4",
        "let downarc = three_fermi_match.matches[0]",
        "let uparc = three_fermi_match.matches[1]",
        "let back = three_fermi_match.matches[2]",
        "require downarc be Some(_)",
        "require downarc!.norm > 3.0",
        "require uparc be Some(_)",
        "let de = downarc!.end_tangent().angle(true)",
        "require de > 0.0 or de < -100.0",
        "let downarc_enpoint = downarc!.end()",
        "let uparc_startpoint = uparc!.start()",
        "let distance = downarc_enpoint.dist(uparc_startpoint)",
        "require distance < 20.0",
        "require three_fermi_match.norm < 2.5",
        "require downarc!.angle_change < -100.0",
        "OneVsAll::Yes",
        "require major_concave_components.ilen() >= 2\n    require major_concave_components.ilen() <= 4\n    let downarc = three_fermi_match.matches[0]\n    let uparc = three_fermi_match.matches[1]\n    let back = three_fermi_match.matches[2]\n    require downarc be Some(_)    \n    require downarc!.norm > 3.0\n    require uparc be Some(_)\n\n    let de = downarc!.end_tangent().angle(true)\n    require de > 0.0 or de < -100.0\n    let downarc_enpoint = downarc!.end()\n    let uparc_startpoint = uparc!.start()\n    let distance = downarc_enpoint.dist(uparc_startpoint)\n    require distance < 20.0\n\n    // to improve\n    require three_fermi_match.norm < 2.5\n    require downarc!.angle_change < -100.0\n\n    OneVsAll::Yes",
    ],
)
```