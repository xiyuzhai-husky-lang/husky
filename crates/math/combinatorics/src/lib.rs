pub fn checked_factorial64(n: u64) -> Option<u64> {
    let mut result = 1u64;
    for i in 1..=n {
        result = result.checked_mul(i)?;
    }
    Some(result)
}

#[test]
fn checked_factorial64_works() {
    fn test(n: u64, expected: u64) {
        assert_eq!(checked_factorial64(n), Some(expected));
    }
    test(0, 1);
    test(1, 1);
    test(2, 2);
    test(3, 6);
    test(4, 24);
}

pub fn checked_binomial_coefficient(n: u128, k: u128) -> Option<u128> {
    if k > n {
        return Some(0);
    }

    // Use smaller k to optimize calculation
    let k = k.min(n - k);

    let mut result = 1u128;
    for i in 0..k {
        // Multiply by (n-i)
        result = result.checked_mul(n - i)?;
        // Divide by (i+1)
        result = result.checked_div(i + 1)?;
    }
    Some(result)
}

#[test]
fn checked_binomial_coefficient_works() {
    fn test(n: u128, k: u128, expected: u128) {
        assert_eq!(checked_binomial_coefficient(n, k), Some(expected));
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
pub fn multinomial_indices(n: u64, r: u64) -> Option<Vec<(u64, Vec<u64>)>> {
    let mut result = Vec::new();

    // We'll build up each combination in `current`, which has `r` slots.
    let mut current = vec![0; r as usize];

    fn backtrack(
        idx: usize,          // which position in `current` we are assigning
        remaining: u64,      // how many "units" of exponent left to distribute
        exponent: u64,       // total exponent n
        current: &mut [u64], // in-progress index-tuple
        result: &mut Vec<(u64, Vec<u64>)>,
    ) -> Option<()> {
        // If we're at the last slot, it must take whatever is left
        if idx == current.len() - 1 {
            current[idx] = remaining;
            let coeff = checked_multinomial_coefficient(exponent, current)?;
            result.push((coeff, current.to_vec()));
            // Reset back to 0 for cleanliness in backtracking
            current[idx] = 0;
            return Some(());
        }

        // Enumerate possible values at this position in *descending* order
        // so that the final output matches the test's expected ordering.
        for val in (0..=remaining).rev() {
            current[idx] = val;
            backtrack(idx + 1, remaining - val, exponent, current, result)?;
            current[idx] = 0;
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
        let mut actual = multinomial_indices(n, r).unwrap();
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

#[track_caller]
pub fn checked_multinomial_coefficient(r: u64, indices: &[u64]) -> Option<u64> {
    // Validate input
    assert_eq!(
        indices.iter().sum::<u64>(),
        r,
        "Sum of indices must equal r for multinomial coefficient, r = {r}, indices = {indices:?}"
    );

    // Start with r!
    let mut result = checked_factorial64(r)?;

    // Divide by the factorial of each index
    for &index in indices {
        result = result.checked_div(checked_factorial64(index)?)?;
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
