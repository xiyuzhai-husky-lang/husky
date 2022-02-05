use rayon::prelude::*;
#[test]
fn increment_all() {
    let mut input = vec![1, 23, 3, 3];
    input.par_iter_mut().for_each(|p| *p += 1);
}
