use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Idx(ShiftedU32);

impl Idx {
    fn new(i: usize) -> Self {
        Self(i.into())
    }
}

impl<T> Seq<Option<T>>
where
    T: Any + Send + Sync + Copy,
{
    pub fn nearest_left(self) -> Seq<Option<(Idx, T)>> {
        let ts = self.data();
        Seq::new(
            (0..ts.len())
                .map(|i| nearest_left_at_index(ts, i))
                .collect(),
        )
    }

    pub fn nearest_right(self) -> Seq<Option<(Idx, T)>> {
        let ts = self.data();
        Seq::new(
            (0..ts.len())
                .map(|i| nearest_right_at_index(ts, i))
                .collect(),
        )
    }
}

fn nearest_left_at_index<T>(ts: &[Option<T>], i: usize) -> Option<(Idx, T)>
where
    T: Any + Send + Sync + Copy,
{
    (1..=i)
        .into_iter()
        .filter_map(|j| ts[(i - j)].map(|t| (Idx::new(i - j), t)))
        .next()
}

fn nearest_right_at_index<T>(ts: &[Option<T>], i: usize) -> Option<(Idx, T)>
where
    T: Any + Send + Sync + Copy,
{
    ((i + 1)..ts.len())
        .into_iter()
        .filter_map(|j| ts[j].map(|t| (Idx::new(j), t)))
        .next()
}

#[test]
fn seq_nearest_left_works() {
    fn t<T>(ts: Seq<Option<T>>, expect: &[Option<(Idx, T)>])
    where
        T: Any + Send + Sync + Copy + Eq + std::fmt::Debug,
    {
        assert_eq!(ts.nearest_left().data(), expect);
    }
    t::<i32>(seq![], &[]);
    t::<i32>(seq![None], &[None]);
    t::<i32>(seq![None, Some(1)], &[None, None]);
    t::<i32>(seq![Some(1), None], &[None, Some((Idx::new(0), 1))]);
    t(
        seq![None, Some(1), Some(2), Some(3), Some(4), None],
        &[
            None,
            None,
            Some((Idx::new(1), 1)),
            Some((Idx::new(2), 2)),
            Some((Idx::new(3), 3)),
            Some((Idx::new(4), 4)),
        ],
    );
}

#[test]
fn seq_nearest_right_works() {
    fn t<T>(ts: Seq<Option<T>>, expect: &[Option<(Idx, T)>])
    where
        T: Any + Send + Sync + Copy + Eq + std::fmt::Debug,
    {
        assert_eq!(ts.nearest_right().data(), expect);
    }
    t::<i32>(seq![], &[]);
    t::<i32>(seq![None], &[None]);
    t::<i32>(seq![None, Some(1)], &[Some((Idx::new(1), 1)), None]);
    t::<i32>(seq![Some(1), None], &[None, None]);
    t(
        seq![None, Some(1), Some(2), Some(3), Some(4), None],
        &[
            Some((Idx::new(1), 1)),
            Some((Idx::new(2), 2)),
            Some((Idx::new(3), 3)),
            Some((Idx::new(4), 4)),
            None,
            None,
        ],
    );
}
