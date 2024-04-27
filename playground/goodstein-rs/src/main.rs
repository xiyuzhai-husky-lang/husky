struct BaseRepr {
    base: usize,
    digits: Vec<usize>,
}

impl BaseRepr {
    fn new(base: usize, input: usize) -> Self {
        if input < base {
            return Self {
                base,
                digits: vec![input],
            };
        }
        let a = input / base;
        let b = input % base;
        let mut slf = Self::new(base, a);
        slf.digits.push(b);
        slf
    }

    fn base_change(&mut self, base: usize) -> &mut Self {
        self.base = base;
        self
    }

    fn calc_value(&self) -> usize {
        let mut s = 0;
        for digit in &self.digits {
            s *= self.base;
            s += digit;
        }
        s
    }
}

#[test]
fn it_works() {
    assert_eq!(BaseRepr::new(10, 111).digits, vec![1, 1, 1]);
    assert_eq!(BaseRepr::new(10, 111).calc_value(), 111);
}

fn main() {
    let list_of_primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    let mut a = 3;
    let mut base_repr = BaseRepr::new(2, a);
    for prime in list_of_primes {
        base_repr.base_change(prime);
        a = base_repr.calc_value() - 1;
        base_repr = BaseRepr::new(prime, a);
        println!("a = {a}");
    }
}
