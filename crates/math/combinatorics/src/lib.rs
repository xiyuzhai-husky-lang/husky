use num_traits::unsigned_int::UnsignedInt;

pub fn checked_binomial_coefficient<N: UnsignedInt>(n: N, k: N) -> Option<N> {
    if k > n {
        return Some(N::ZERO);
    }

    // Use smaller k to optimize calculation
    let k = k.min(n.checked_sub(k)?);

    let mut result = N::ONE;
    for i in 0..k.as_usize() {
        let i = N::from_usize(i);
        // Multiply by (n-i)
        result = result.checked_mul(n.checked_sub(i)?)?;
        // Divide by (i+1)
        result = result.checked_div(i.checked_add(N::ONE)?)?;
    }
    Some(result)
}

#[test]
fn checked_binomial_coefficient_works() {
    fn test(n: u128, k: u128, expected: u128) {
        assert_eq!(checked_binomial_coefficient::<u128>(n, k), Some(expected));
    }

    test(0, 0, 1);
    test(1, 0, 1);
    test(1, 1, 1);
    test(5, 2, 10);
    test(10, 3, 120);
    test(20, 10, 184756);
    test(5, 6, 0); // k > n case
}

/// For sum of `r` variables raised to the `n`th power (interpreting the signature as
/// `multinomial_indices(n, r)`, i.e., n = exponent, r = number_of_variables),
/// return the coefficients of its multinomial expansion along with their index-tuples.
pub fn multinomial_expansion<N: UnsignedInt>(
    n: N,
    r: N,
    max_size: usize,
) -> Option<Vec<(N, Vec<N>)>> {
    // Check if r exceeds vector capacity
    if r.as_usize() > max_size {
        return None;
    }

    // Check if total combinations would exceed capacity
    let terms = count_expansion_terms(n, r)?;
    if terms.as_usize() > max_size {
        return None;
    }

    let mut result = Vec::new();
    let mut current = vec![N::ZERO; r.as_usize()];

    fn backtrack<N: UnsignedInt>(
        idx: usize,        // which position in `current` we are assigning
        remaining: N,      // how many "units" of exponent left to distribute
        exponent: N,       // total exponent n
        current: &mut [N], // in-progress index-tuple
        result: &mut Vec<(N, Vec<N>)>,
    ) -> Option<()> {
        // If we're at the last slot, it must take whatever is left
        if idx == current.len() - 1 {
            current[idx] = remaining;
            let coeff = checked_multinomial_coefficient(exponent, current)?;
            result.push((coeff, current.to_vec()));
            // Reset back to 0 for cleanliness in backtracking
            current[idx] = N::ZERO;
            return Some(());
        }

        // Enumerate possible values at this position in *descending* order
        // so that the final output matches the test's expected ordering.
        for val in (0..=remaining.as_usize()).rev() {
            let val = N::from_usize(val);
            current[idx] = val;
            backtrack(
                idx + 1,
                remaining.checked_sub(val)?,
                exponent,
                current,
                result,
            )?;
            current[idx] = N::ZERO;
        }
        Some(())
    }

    backtrack(0, n, n, &mut current, &mut result)?;
    Some(result)
}

#[test]
fn multinomial_indices_works() {
    #[track_caller]
    fn t(n: u64, r: u64, expected: Vec<(u64, Vec<u64>)>) {
        let mut actual = multinomial_expansion(n, r, 40).unwrap();
        assert_eq!(actual, expected);
    }

    // (x + y)^2 = x^2 + 2xy + y^2
    t(
        2,
        2,
        vec![
            (1, vec![2, 0]), // x^2
            (2, vec![1, 1]), // 2xy
            (1, vec![0, 2]), // y^2
        ],
    );

    // (x + y + z)^2
    t(
        2,
        3,
        vec![
            (1, vec![2, 0, 0]), // x^2
            (2, vec![1, 1, 0]), // 2xy
            (2, vec![1, 0, 1]), // 2xz
            (1, vec![0, 2, 0]), // y^2
            (2, vec![0, 1, 1]), // 2yz
            (1, vec![0, 0, 2]), // z^2
        ],
    );

    // (x + y + z)^3
    t(
        3,
        3,
        vec![
            (1, vec![3, 0, 0]), // x^3
            (3, vec![2, 1, 0]), // 3x^2y
            (3, vec![2, 0, 1]), // 3x^2z
            (3, vec![1, 2, 0]), // 3xy^2
            (6, vec![1, 1, 1]), // 6xyz
            (3, vec![1, 0, 2]), // 3xz^2
            (1, vec![0, 3, 0]), // y^3
            (3, vec![0, 2, 1]), // 3y^2z
            (3, vec![0, 1, 2]), // 3yz^2
            (1, vec![0, 0, 3]), // z^3
        ],
    );

    // (x + y)^3
    t(
        3,
        2,
        vec![
            (1, vec![3, 0]), // x^3
            (3, vec![2, 1]), // 3x^2y
            (3, vec![1, 2]), // 3xy^2
            (1, vec![0, 3]), // y^3
        ],
    );

    // (x + y + z + w)^2
    t(
        2,
        4,
        vec![
            (1, vec![2, 0, 0, 0]), // x^2
            (2, vec![1, 1, 0, 0]), // 2xy
            (2, vec![1, 0, 1, 0]), // 2xz
            (2, vec![1, 0, 0, 1]), // 2xw
            (1, vec![0, 2, 0, 0]), // y^2
            (2, vec![0, 1, 1, 0]), // 2yz
            (2, vec![0, 1, 0, 1]), // 2yw
            (1, vec![0, 0, 2, 0]), // z^2
            (2, vec![0, 0, 1, 1]), // 2zw
            (1, vec![0, 0, 0, 2]), // w^2
        ],
    );
}

#[test]
fn multinomial_indices_overflows() {
    #[track_caller]
    fn t(n: u64, r: u64) {
        assert!(multinomial_expansion(n, r, 100).is_none());
    }

    // Large numbers that should overflow
    t(100, 100);
    t(u64::MAX, 2); // Extreme n value
    t(2, u64::MAX); // Extreme r value
    t(1000, 50); // Large n with moderate r
    t(50, 1000); // Moderate n with large r

    // Edge cases that should overflow
    t(64, 64); // Numbers near u64 bit size
    t(u64::MAX - 1, 2); // Near maximum u64 value
    t(2, u64::MAX - 1); // Near maximum u64 value
}

#[track_caller]
pub fn checked_multinomial_coefficient<N: UnsignedInt>(r: N, indices: &[N]) -> Option<N> {
    // Validate input
    assert_eq!(
        indices.iter().copied().sum::<N>(),
        r,
        "Sum of indices must equal r for multinomial coefficient, r = {r}, indices = {indices:?}"
    );

    // Start with r!
    let mut result = r.checked_factorial()?;

    // Divide by the factorial of each index
    for &index in indices {
        result = result.checked_div(index.checked_factorial()?)?;
    }

    Some(result)
}

#[test]
fn checked_multinomial_coefficient_works() {
    fn test(n: u64, indices: &[u64], expected: u64) {
        assert_eq!(checked_multinomial_coefficient(n, indices), Some(expected));
    }

    // Basic cases
    test(3, &[1, 2], 3); // 3!/(1!2!)
    test(4, &[2, 2], 6); // 4!/(2!2!)
    test(3, &[1, 1, 1], 6); // 3!/(1!1!1!)
    test(4, &[1, 1, 2], 12); // 4!/(1!1!2!)

    // Edge cases
    test(0, &[0], 1);
    test(1, &[1], 1);
    test(2, &[0, 2], 1);
}

fn count_expansion_terms<N: UnsignedInt>(n: N, r: N) -> Option<N> {
    // Formula is binomial(n+r-1,r-1) for combinations with repetition
    let n_plus_r = n.checked_add(r)?;
    let n_plus_r_minus_1 = n_plus_r.checked_sub(N::ONE)?;
    let r_minus_1 = r.checked_sub(N::ONE)?;
    checked_binomial_coefficient(n_plus_r_minus_1, r_minus_1)
}
