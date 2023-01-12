Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::raw_contour`,
        module_specific_symbols: [
            ModuleItem {
                ident: `RawContour`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 203,
                path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            },
            ModuleItem {
                ident: `Direction`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 205,
                path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
            },
            ModuleItem {
                ident: `get_pixel_pair`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 210,
                path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
            },
            ModuleItem {
                ident: `get_pixel_to_the_left`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 211,
                path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
            },
            ModuleItem {
                ident: `get_pixel_to_the_right`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 212,
                path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
            },
            ModuleItem {
                ident: `get_inward_direction`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 213,
                path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
            },
            ModuleItem {
                ident: `get_angle_change`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 214,
                path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
            },
            ModuleItem {
                ident: `get_outward_direction`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 215,
                path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
            },
            ModuleItem {
                ident: `StreakCache`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 216,
                path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            },
            ModuleItem {
                ident: `get_concave_middle_point`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 217,
                path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
            },
            ModuleItem {
                ident: `find_raw_contours`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 218,
                path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 199,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        1..2,
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 200,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        4..5,
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 201,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        7..8,
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 202,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        9..11,
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)