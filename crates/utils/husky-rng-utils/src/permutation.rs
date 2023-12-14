pub fn generate_random_permutation(n: u32, seed: u64) -> Vec<u32> {
    use rand::prelude::StdRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    let mut vec: Vec<u32> = (0..n).collect();
    vec.shuffle(&mut StdRng::seed_from_u64(seed));
    vec
}
