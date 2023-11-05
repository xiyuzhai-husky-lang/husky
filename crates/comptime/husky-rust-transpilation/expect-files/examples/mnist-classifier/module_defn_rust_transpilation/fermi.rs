
struct FermiMatchResult{ matches : Vec< Option< Leash< ConcaveComponent>>>, others : Vec< Leash< ConcaveComponent>>}

pub fn fermi_match {
    let others = concave_components.collect_leashes();
    let matches : Vec< Option< Leash< ConcaveComponent>>> = vec![]; for {
        let template = templates[ i];
        matches.push( others.pop_with_largest_opt_f32( template));
    } return( FermiMatchResult( matches, others))
}

impl {
    fn norm(self) {
        let norm : f32 =0; for {
            norm= norm.max(Self.others[ i].norm);
        } return( norm)
    }

    fn rel_norm(self) {
        let norm : f32 =0; for {
            norm= norm.max(Self.others[ i].rel_norm);
        } return( norm)
    }

    fn angle_change_norm(self) {
        let norm : f32 =0; for {
            norm= norm.max(Self.others[ i].angle_change.abs());
        } return( norm)
    }
}

fn norm(self) {
    let norm : f32 =0; for {
        norm= norm.max(Self.others[ i].norm);
    } return( norm)
}

fn rel_norm(self) {
    let norm : f32 =0; for {
        norm= norm.max(Self.others[ i].rel_norm);
    } return( norm)
}

fn angle_change_norm(self) {
    let norm : f32 =0; for {
        norm= norm.max(Self.others[ i].angle_change.abs());
    } return( norm)
}