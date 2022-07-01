use crate::*;

pub struct RawContour<'eval> {
    pub(crate) cc: &'eval crate::connected_component::ConnectedComponent,
    pub(crate) points: Vec<crate::geom2d::Point2d>,
}

impl<'eval> RawContour<'eval> {
    pub(crate) fn displacement(&self, start: i32, end: i32) -> crate::geom2d::Vector2d {
        let N = self.points.ilen();
        let ct_start = self.points[start % N];
        let ct_end = self.points[end % N];
        return ct_start.to(&ct_end)
    }
}
enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

pub(crate) fn get_pixel_pair(row: u32, j: i32) -> u32 {
    return (row >> (j - 1i32)) & 3u32
}

pub(crate) fn get_pixel_to_the_left(row: u32, j: i32) -> u32 {
    return (row >> j) & 1u32
}

pub(crate) fn get_pixel_to_the_right(row: u32, j: i32) -> u32 {
    return (row >> (j - 1i32)) & 1u32
}

pub(crate) fn get_inward_direction(row_above: u32, row_below: u32, j: i32) -> crate::raw_contour::Direction {
    let pixel_pair_above = crate::raw_contour::get_pixel_pair(row_above, j);
    let pixel_pair_below = crate::raw_contour::get_pixel_pair(row_below, j);
    match pixel_pair_above {
        0 => {
            match pixel_pair_below {
                1 | 3 => {
                    return crate::raw_contour::Direction::LEFT
                }
                2 => {
                    return crate::raw_contour::Direction::UP
                }
                _ => panic!(),
            }
        }
        1 => {
            return crate::raw_contour::Direction::DOWN
        }
        2 => {
            match pixel_pair_below {
                0 => {
                    return crate::raw_contour::Direction::RIGHT
                }
                1 | 3 => {
                    return crate::raw_contour::Direction::LEFT
                }
                2 => {
                    return crate::raw_contour::Direction::UP
                }
                _ => panic!(),
            }
        }
        3 => {
            match pixel_pair_below {
                0 | 1 => {
                    return crate::raw_contour::Direction::RIGHT
                }
                2 => {
                    return crate::raw_contour::Direction::UP
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

pub(crate) fn get_angle_change(inward: crate::raw_contour::Direction, outward: crate::raw_contour::Direction) -> i32 {
    let raw_angle_change = (((outward as i32) - (inward as i32)) as b32).last_bits(2i32);
    match raw_angle_change {
        0 | 1 | 2 => {
            return raw_angle_change as i32
        }
        3 => {
            return -1i32
        }
        _ => panic!(),
    }
}

pub(crate) fn get_outward_direction(row_above: u32, row_below: u32, j: i32, inward_direction: crate::raw_contour::Direction) -> crate::raw_contour::Direction {
    let pixel_pair_above = crate::raw_contour::get_pixel_pair(row_above, j);
    let pixel_pair_below = crate::raw_contour::get_pixel_pair(row_below, j);
    match pixel_pair_above {
        0 => {
            match pixel_pair_below {
                1 => {
                    return crate::raw_contour::Direction::DOWN
                }
                2 | 3 => {
                    return crate::raw_contour::Direction::LEFT
                }
                _ => panic!(),
            }
        }
        1 => {
            match pixel_pair_below {
                0 => {
                    return crate::raw_contour::Direction::RIGHT
                }
                1 => {
                    return crate::raw_contour::Direction::DOWN
                }
                2 => {
                    match inward_direction {
                        crate::raw_contour::Direction::DOWN => {
                            return crate::raw_contour::Direction::LEFT
                        }
                        crate::raw_contour::Direction::UP => {
                            return crate::raw_contour::Direction::RIGHT
                        }
                        _ => panic!(),
                    }
                }
                3 => {
                    return crate::raw_contour::Direction::LEFT
                }
                _ => panic!(),
            }
        }
        2 => {
            match pixel_pair_below {
                0 | 2 | 3 => {
                    return crate::raw_contour::Direction::UP
                }
                1 => {
                    match inward_direction {
                        crate::raw_contour::Direction::LEFT => {
                            return crate::raw_contour::Direction::UP
                        }
                        crate::raw_contour::Direction::RIGHT => {
                            return crate::raw_contour::Direction::DOWN
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
        3 => {
            match pixel_pair_below {
                0 | 2 => {
                    return crate::raw_contour::Direction::RIGHT
                }
                1 => {
                    return crate::raw_contour::Direction::DOWN
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}
pub struct StreakCache {
    pub(crate) prev1: i32,
    pub(crate) prev2: i32,
}

impl StreakCache {
}

pub(crate) fn get_concave_middle_point(points: &Vec<crate::geom2d::Point2d>) -> crate::geom2d::Point2d {
    let N = points.ilen();
    let p0 = points[N - 2i32];
    let p2 = points[N - 1i32];
    return crate::geom2d::Point2d::__call__((p0.x + p2.x) / 2f32, (p0.y + p2.y) / 2f32)
}

pub(crate) fn find_raw_contours(cc: &'eval crate::connected_component::ConnectedComponent) -> Vec<crate::raw_contour::RawContour<'eval>> {
    let mut result = Vec::<crate::raw_contour::RawContour<'eval>>::__call__();
    let mut boundary_unsearched = domains::ml::datasets::cv::mnist::BinaryGrid28::__call__();
    for i in 1i32..(29i32 + 1) {
        let r_ur = cc.mask[i - 1i32];
        let r_dr = cc.mask[i];
        let r_ul = r_ur << 1i32;
        let r_dl = r_dr << 1i32;
        boundary_unsearched[i] = (r_ur | r_dr | r_ul | r_dl) & (!(r_ur & r_dr & r_ul & r_dl));
    }

    for k in 1i32..(29i32 + 1) {
        while boundary_unsearched[k] != 0 {
            let mut contour = Vec::<crate::geom2d::Point2d>::__call__();
            let mut i = k;
            let mut j = boundary_unsearched[k].trailing_zeros();
            let mut row_above = cc.mask[i - 1i32];
            let mut row_below = cc.mask[i];
            let mut inward_direction = crate::raw_contour::get_inward_direction(row_above, row_below, j);
            let i0 = i;
            let j0 = j;
            let dir0 = inward_direction;
            let mut prev_angle_change1 = 0i32;
            let mut prev_angle_change2 = 0i32;
            let mut total_angle_change = 0i32;
            let mut prev_streak1 = -1i32;
            let mut prev_streak2 = -1i32;
            let mut current_streak = -1i32;
            loop {
                let outward_direction = crate::raw_contour::get_outward_direction(row_above, row_below, j, inward_direction);
                let angle_change = crate::raw_contour::get_angle_change(inward_direction, outward_direction);
                boundary_unsearched[i] = boundary_unsearched[i] & (!(1u32 << j));
                if angle_change != 0 {
                    if prev_angle_change1 == -1i32 && prev_angle_change2 == -1i32 && current_streak == 1i32 && prev_streak1 != -1i32 && prev_streak2 == 1i32 {
                        contour.last() = crate::raw_contour::get_concave_middle_point(&contour);
                        contour.push(crate::geom2d::Point2d::from_i_shift28(i, j));
                        prev_streak2 = -1i32;
                        prev_streak1 = -1i32;
                    } else if prev_angle_change1 == -1i32 && prev_streak1 > 0i32 && prev_streak1 == 1i32 {
                        contour.last() = crate::geom2d::Point2d::from_i_shift28(i, j);
                        prev_streak2 = prev_streak1;
                        prev_streak1 = current_streak;
                    } else if prev_angle_change1 == -1i32 && prev_streak1 > 0i32 && current_streak == 1i32 && prev_streak1 > 1i32 {
                        contour.last() = crate::geom2d::Point2d::from_i_shift28(i, j);
                        prev_streak2 = -1i32;
                        prev_streak1 = -1i32;
                    } else {
                        contour.push(crate::geom2d::Point2d::from_i_shift28(i, j));
                        prev_streak2 = prev_streak1;
                        prev_streak1 = current_streak;
                    }
                    current_streak = 0i32;
                    prev_angle_change2 = prev_angle_change1;
                    prev_angle_change1 = angle_change;
                }
                match outward_direction {
                    crate::raw_contour::Direction::UP => {
                        i = i - 1i32;
                        row_below = row_above;
                        row_above = cc.mask[i - 1i32];
                    }
                    crate::raw_contour::Direction::DOWN => {
                        i = i + 1i32;
                        row_above = row_below;
                        row_below = cc.mask[i];
                    }
                    crate::raw_contour::Direction::LEFT => {
                        j = j + 1i32;
                    }
                    crate::raw_contour::Direction::RIGHT => {
                        j = j - 1i32;
                    }
                _ => panic!(),
                }
                inward_direction = outward_direction;
                if current_streak != -1i32 {
                    current_streak += 1;
                }
                if !(!(i == i0 && j == j0 && inward_direction == dir0)) {
                    break;
                }
            }

            if prev_angle_change1 == -1i32 && current_streak == 1i32 && prev_streak1 > 0i32 {
                contour.pop();
            }
            result.push(crate::raw_contour::RawContour::<'eval>::__call__(cc, contour));
        }

    }

    return result
}
