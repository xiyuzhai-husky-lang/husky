```rust
Some(
    [
        "let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "if major_connected_component.eff_holes.matches[1] be None:\n        if major_connected_component.eff_holes.matches[0] be None:\n            require false\n        require false",
        "OneVsAll::Yes",
        "let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n         \n    // require upper_excess>-5.0 and upper_excess<37.0\n\n    // if major_connected_component.max_hole_ilen == 2.0:\n    //     MnistLabel::Eight\n\n    if major_connected_component.eff_holes.matches[1] be None:\n        if major_connected_component.eff_holes.matches[0] be None:\n            require false\n        require false\n    OneVsAll::Yes",
    ],
)
```