struct FermiMatchResult{
    matches: Vec<Option<Leash<ConcaveComponent>>>,
    others: Vec<Leash<ConcaveComponent>>,
}

pub fn fermi_match(concave_components: Leash<Vec<ConcaveComponent>>, templates: Vec< HirTypeRitchieTodo >) -> FermiMatchResult {
    let mut others = concave_components.collect_leashes();
    let mut matches: Vec<Option<Leash<ConcaveComponent>>> = vec![];
    for i in 0..templates.ilen() {
        let template = templates[i];
        matches.push(others.pop_with_largest_opt_f32(template))
    }
    return FermiMatchResult(matches, others);
}

impl FermiMatchResult {
    fn norm(self) {
        let mut norm: f32 = 0;
        for i in 0..self.others.ilen() {
            norm = norm.max(self.others[i].norm)
        }
        return norm;
    }

    fn rel_norm(self) {
        let mut norm: f32 = 0;
        for i in 0..self.others.ilen() {
            norm = norm.max(self.others[i].rel_norm)
        }
        return norm;
    }

    fn angle_change_norm(self) {
        let mut norm: f32 = 0;
        for i in 0..self.others.ilen() {
            norm = norm.max(self.others[i].angle_change.abs())
        }
        return norm;
    }
}