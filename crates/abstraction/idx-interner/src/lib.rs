use std::marker::PhantomData;

use husky_print_utils::{epin, p};

pub trait Internable: PartialEq + Eq + Sized + std::fmt::Debug {
    fn max_dependee_idx(&self) -> Option<InternIdx<Self>>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternIdx<T>
where
    T: Internable,
{
    raw: usize,
    phantom: PhantomData<T>,
}

impl<T> Clone for InternIdx<T>
where
    T: Internable,
{
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            phantom: self.phantom.clone(),
        }
    }
}
impl<T> Copy for InternIdx<T> where T: Internable {}

impl<T> InternIdx<T>
where
    T: Internable,
{
    pub(crate) fn new(raw: usize) -> Self {
        InternIdx {
            raw,
            phantom: PhantomData,
        }
    }

    pub fn raw(&self) -> usize {
        self.raw
    }
}

#[derive(Debug)]
pub struct Interner<T>
where
    T: Internable,
{
    data: Vec<T>,
}

impl<T> Interner<T>
where
    T: Internable,
{
    pub fn new() -> Self {
        Interner { data: vec![] }
    }

    pub fn it(&mut self, t: T) -> InternIdx<T> {
        // try search for same element
        if let Some(max_dependee_idx) = t.max_dependee_idx() {
            let search_start = max_dependee_idx.raw;
            assert!(search_start <= self.data.len());
            for (i, s) in self.data[search_start..].iter().enumerate() {
                if s == &t {
                    return InternIdx::new(search_start + i);
                }
            }
        }
        let raw = self.data.len();
        self.data.push(t);
        InternIdx::new(raw)
    }
}

impl<T> Default for Interner<T>
where
    T: Internable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::ops::Index<InternIdx<T>> for Interner<T>
where
    T: Internable,
{
    type Output = T;

    fn index(&self, index: InternIdx<T>) -> &Self::Output {
        &self.data[index.raw]
    }
}

#[test]
fn idx_intern_works() {
    #[derive(Debug, PartialEq, Eq)]
    enum Nat {
        Zero,
        Succ(NatIdx),
    };

    impl Internable for Nat {
        fn max_dependee_idx(&self) -> Option<InternIdx<Self>> {
            match self {
                Nat::Zero => None,
                Nat::Succ(nat) => Some(*nat),
            }
        }
    }

    type NatIdx = InternIdx<Nat>;
    type NatInterner = Interner<Nat>;

    let mut interner = NatInterner::new();
    let zero = interner.it(Nat::Zero);
    assert_eq!(zero.raw(), 0);
    let one = interner.it(Nat::Succ(zero));
    assert_eq!(one.raw(), 1);
    let two = interner.it(Nat::Succ(one));
    assert_eq!(interner[two].max_dependee_idx().unwrap().raw(), 1);
    assert_eq!(two.raw(), 2);
    assert_eq!(interner.it(Nat::Succ(one)).raw(), 2);
    assert_eq!(interner.it(Nat::Succ(zero)).raw(), 1);
}
