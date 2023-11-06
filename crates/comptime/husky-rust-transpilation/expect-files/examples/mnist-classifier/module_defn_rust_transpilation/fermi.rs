
struct FermiMatchResult{matches: Vec<Option<Leash<ConcaveComponent>>>, others: Vec<Leash<ConcaveComponent>>}

pub fn fermi_match(concave_components: Leash<Vec<ConcaveComponent>>, templates: Vec< HirTypeRitchieTodo >) -> FermiMatchResult {
    let others = concave_components.collect_leashes();
    let matches: Vec<Option<Leash<ConcaveComponent>>> = vec![];
    for {
        let template = templates[i];
        matches.push(others.pop_with_largest_opt_f32(template));
    }
    return FermiMatchResult(matches, others);
}