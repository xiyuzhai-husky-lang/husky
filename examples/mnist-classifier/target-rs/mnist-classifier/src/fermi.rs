use crate::*;

pub struct FermiMatchResult {
    pub matches: Vec<Option<Leash<ConcaveComponent>>>,
    pub others: Vec<Leash<ConcaveComponent>>,
} 

pub fn fermi_match(concave_components: Leash<Vec<ConcaveComponent>>, templates: &Vec<fn(Leash<ConcaveComponent>) -> Option<f32>>) -> FermiMatchResult {
    let mut others = concave_components.collect_leashes();
    let mut matches: Vec<Option<Leash<ConcaveComponent>>> = vec![];
    for i in 0..templates.ilen() {
        let template = templates[i];
        matches.push(others.pop_with_largest_opt_f32(template))
    }
    return FermiMatchResult::__constructor(matches, others);
}

impl FermiMatchResult {
    pub fn norm(self) -> f32 {
        let mut norm: f32 = 0.0f32;
        for i in 0..self.others.ilen() {
            norm = norm.max(self.others[i].norm())
        }
        return norm;
    }

    pub fn rel_norm(self) -> f32 {
        let mut norm: f32 = 0.0f32;
        for i in 0..self.others.ilen() {
            norm = norm.max(self.others[i].rel_norm())
        }
        return norm;
    }

    pub fn angle_change_norm(self) -> f32 {
        let mut norm: f32 = 0.0f32;
        for i in 0..self.others.ilen() {
            norm = norm.max(self.others[i].angle_change().abs())
        }
        return norm;
    }
}