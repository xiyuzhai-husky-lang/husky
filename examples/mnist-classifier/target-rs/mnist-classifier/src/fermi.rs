use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FermiMatchResult {
    pub matches: Vec<Option<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>>,
    pub others: Vec<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>,
}

impl FermiMatchResult {
    pub fn __constructor(matches: Vec<Option<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>>, others: Vec<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>) -> Self {
        Self{
            matches,
            others,
        }
    }
}

pub fn fermi_match(concave_components: Leash<Vec<crate::line_segment_sketch::concave_component::ConcaveComponent>>, templates: &Vec<fn(Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32>>) -> crate::fermi::FermiMatchResult {
    let mut others = concave_components.collect_leashes();
    let mut matches: Vec<Option<Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>>> = vec![];
    for i in 0..templates.ilen() {
        let template = templates[i as usize];
        matches.push(others.pop_with_largest_opt_f32(template))
    }
    return crate::fermi::FermiMatchResult::__constructor(matches, others);
}

impl crate::fermi::FermiMatchResult {
    #[ad_hoc_task_dependency::memoized_field(21)]
pub fn norm(&'static self) -> f32 {
        let mut norm: f32 = 0.0f32;
        for i in 0..self.others.ilen() {
            norm = norm.max(self.others[i as usize].norm())
        }
        return norm;
    }

    #[ad_hoc_task_dependency::memoized_field(22)]
pub fn rel_norm(&'static self) -> f32 {
        let mut norm: f32 = 0.0f32;
        for i in 0..self.others.ilen() {
            norm = norm.max(self.others[i as usize].rel_norm())
        }
        return norm;
    }

    #[ad_hoc_task_dependency::memoized_field(23)]
pub fn angle_change_norm(&'static self) -> f32 {
        let mut norm: f32 = 0.0f32;
        for i in 0..self.others.ilen() {
            norm = norm.max(self.others[i as usize].angle_change().abs())
        }
        return norm;
    }
}