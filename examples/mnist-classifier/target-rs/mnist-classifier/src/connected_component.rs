use crate::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectedComponentDistribution {
    pub row_start: i32,
    pub row_end: i32,
    pub upper_mass: i32,
    pub lower_mass: i32,
}

impl ConnectedComponentDistribution {
    pub fn __constructor(row_start: i32, row_end: i32, upper_mass: i32, lower_mass: i32) -> Self {
        Self{
            row_start,
            row_end,
            upper_mass,
            lower_mass,
        }
    }
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct EffHoles {
    pub matches: Vec<Option<Leash<crate::raw_contour::RawContour>>>,
}

impl EffHoles {
    pub fn __constructor(matches: Vec<Option<Leash<crate::raw_contour::RawContour>>>) -> Self {
        Self{
            matches,
        }
    }
}

#[rustfmt::skip]
pub fn hole_tmpl(ct: Leash<crate::raw_contour::RawContour>) -> Option<f32> {
    let len = ct.contour_len();
    require!(len > 4.0f32);
    Some(len + 0.0f32)
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectedComponent {
    pub mask: mnist::BinaryImage28,
}

impl ConnectedComponent {
    pub fn __constructor(mask: mnist::BinaryImage28) -> Self {
        Self{
            mask,
        }
    }
}

#[rustfmt::skip]
pub fn horizontal_extend(a: u32, x: u32) -> u32 {
    let mut y = a & (x | x << 1 | x >> 1);
    let mut z = a & (y | y << 1 | y >> 1);
    while z != y {
        y = z;
        z = a & (y | y << 1 | y >> 1)
    }
    return y;
}

#[rustfmt::skip]
pub fn find_connected_components(img: &mnist::BinaryImage28) -> Vec<crate::connected_component::ConnectedComponent> {
    let mut result: Vec<crate::connected_component::ConnectedComponent> = vec![];
    let mut unsearched = img.clone();
    for j in 0..30 {
        while unsearched[j as usize] != 0 {
            let a = unsearched[j as usize];
            let shift = a.ctz();
            let mut mask = mnist::BinaryImage28::new_zeros();
            mask[j as usize] = crate::connected_component::horizontal_extend(a, 1 << shift);
            let mut flag = false;
            while !flag {
                flag = true;
                let mut i = j;
                while i < 30 - 1 {
                    {
                        let old_row = mask[(i + 1) as usize];
                        let new_row = old_row | crate::connected_component::horizontal_extend(img[(i + 1) as usize], mask[i as usize]);
                        if (new_row == 0) {
                            break;
                        }
                        if old_row != new_row {
                            flag = false;
                            mask[(i + 1) as usize] = new_row
                        }
                    }
                    i += 1
                }
                while i >= j {
                    {
                        let old_row = mask[i as usize];
                        let new_row = old_row | crate::connected_component::horizontal_extend(img[i as usize], mask[(i + 1) as usize]);
                        if old_row != new_row {
                            flag = false;
                            mask[i as usize] = new_row
                        }
                    }
                    i -= 1
                }
            }
            for k in j..30 {
                unsearched[k as usize] &= !mask[k as usize]
            }
            result.push(crate::connected_component::ConnectedComponent::__constructor(mask))
        }
    }
    return result;
}

#[rustfmt::skip]
impl Visualize for crate::connected_component::ConnectedComponent {
    fn visualize(&self, __visual_synchrotron: &mut __VisualSynchrotron) -> husky_core::visual::Visual {
        self.mask.visualize(__visual_synchrotron)
    }
}

#[rustfmt::skip]
impl crate::connected_component::ConnectedComponent {
    #[ad_hoc_task_dependency::memo_field(ingredient_index = 1, return_ref)]
    pub fn raw_contours(&'static self) -> Vec<crate::raw_contour::RawContour> {
        crate::raw_contour::find_raw_contours(&self)
    }

    #[ad_hoc_task_dependency::memo_field(ingredient_index = 2, return_ref)]
    pub fn eff_holes(&'static self) -> crate::connected_component::EffHoles {
        let mut raw_contours = self.raw_contours().collect_leashes();
        let mut matches: Vec<Option<Leash<crate::raw_contour::RawContour>>> = vec![];
        raw_contours.pop_with_largest_opt_f32(hole_tmpl);
        matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl));
        matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl));
        return crate::connected_component::EffHoles::__constructor(matches);
    }

    #[ad_hoc_task_dependency::memo_field(ingredient_index = 3)]
    pub fn max_hole_ilen(&'static self) -> f32 {
        let mut max_hole_ilen = 0;
        let raw_contours = &self.raw_contours();
        for i in (0 + 1)..raw_contours.ilen() {
            let hole_ilen = raw_contours[i as usize].points.ilen();
            if max_hole_ilen < hole_ilen {
                max_hole_ilen = hole_ilen
            }
        }
        return max_hole_ilen as f32;
    }

    #[ad_hoc_task_dependency::memo_field(ingredient_index = 4)]
    pub fn max_row_span(&'static self) -> f32 {
        let mut max_row: i32 = 0;
        for i in (0 + 1)..29 {
            max_row = max_row.max(self.mask[i as usize].span())
        }
        return max_row as f32;
    }

    #[ad_hoc_task_dependency::memo_field(ingredient_index = 5)]
    pub fn row_span_sum(&'static self) -> f32 {
        let mut row_span_sum = 0;
        for i in (0 + 1)..29 {
            row_span_sum += self.mask[i as usize].span()
        }
        return row_span_sum as f32;
    }

    #[ad_hoc_task_dependency::memo_field(ingredient_index = 6, return_ref)]
    pub fn distribution(&'static self) -> crate::connected_component::ConnectedComponentDistribution {
        let mut row_start = 1;
        while row_start < 29 {
            {
                if self.mask[row_start as usize] != 0 {
                    break;
                }
            }
            row_start += 1
        }
        let mut row_end = row_start + 1;
        while row_end < 29 {
            {
                if (self.mask[row_end as usize] == 0) {
                    break;
                }
            }
            row_end += 1
        }
        let height = row_end - row_start;
        let half_height = height / 2;
        let mut upper_mass = 0;
        for i1 in row_start..row_start + half_height {
            upper_mass += self.mask[i1 as usize].co()
        }
        let mut lower_mass = 0;
        for i2 in (row_end - half_height..row_end).rev() {
            lower_mass += self.mask[i2 as usize].co()
        }
        return crate::connected_component::ConnectedComponentDistribution::__constructor(row_start, row_end, upper_mass, lower_mass);
    }

    #[ad_hoc_task_dependency::memo_field(ingredient_index = 7)]
    pub fn upper_mass(&'static self) -> f32 {
        self.distribution().upper_mass as f32
    }

    #[ad_hoc_task_dependency::memo_field(ingredient_index = 8)]
    pub fn lower_mass(&'static self) -> f32 {
        self.distribution().lower_mass as f32
    }

    pub fn top_k_row_span_sum(&self, k: i32) -> f32 {
        let mut top_k_row_span_sum = 0;
        assert!(k > 0);
        let mut i = 1;
        while i < 29 {
            {
                if self.mask[i as usize] != 0 {
                    break;
                }
            }
            i += 1
        }
        for j in i..i + k {
            top_k_row_span_sum += self.mask[j as usize].span()
        }
        return top_k_row_span_sum as f32;
    }

    pub fn top_k_row_right_mass_sum(&self, k: i32) -> f32 {
        let mut top_k_row_span_sum = 0;
        assert!(k > 0);
        let mut i = 1;
        while i < 29 {
            {
                if self.mask[i as usize] != 0 {
                    break;
                }
            }
            i += 1
        }
        for j in i..i + k {
            top_k_row_span_sum += self.mask[j as usize].right_mass()
        }
        return top_k_row_span_sum as f32;
    }
}