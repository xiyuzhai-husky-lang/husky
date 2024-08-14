use super::*;

impl<T> Seq<T>
where
    T: Any + Send + Sync + Copy,
{
    pub fn nearest_left_filtered_by_attention<Q, K>(
        self,
        qs: Seq<Q>,
        ks: Seq<K>,
        f: impl Fn(Q, K) -> bool,
    ) -> Seq<Option<T>>
    where
        Q: Any + Send + Sync + Copy,
        K: Any + Send + Sync + Copy,
    {
        let slf = &self.data();
        let len = slf.len();
        let qs = qs.data();
        let ks = ks.data();
        Seq::new(
            (0..len)
                .into_iter()
                .map(|i| {
                    (1..=i)
                        .into_iter()
                        .filter_map(|j| f(qs[i], ks[i - j]).then_some(slf[i - j]))
                        .next()
                })
                .collect(),
        )
    }

    pub fn first_filtered_by_attention<Q, K>(
        self,
        qs: Seq<Q>,
        ks: Seq<K>,
        f: impl Fn(Q, K) -> bool,
    ) -> Seq<Option<T>>
    where
        Q: Any + Send + Sync + Copy,
        K: Any + Send + Sync + Copy,
    {
        let slf = &self.data();
        let len = slf.len();
        let qs = qs.data();
        let ks = ks.data();
        Seq::new(
            (0..len)
                .into_iter()
                .map(|i| {
                    (0..len)
                        .into_iter()
                        .filter_map(|j| f(qs[i], ks[j]).then_some(slf[j]))
                        .next()
                })
                .collect(),
        )
    }
}

#[test]
fn nearest_left_filtered_by_attention_works() {
    let xs = seq![1, 2];
    assert_eq!(
        xs.nearest_left_filtered_by_attention(seq![1, 2], seq![1, 2], |q, k| q == k),
        seq![None, None]
    );
    assert_eq!(
        xs.nearest_left_filtered_by_attention(seq![1, 2], seq![2, 1], |q, k| q == k),
        seq![None, Some(1)]
    );
}

#[test]
fn first_filtered_by_attention_works() {
    let xs = seq![1, 2];
    assert_eq!(
        xs.first_filtered_by_attention(seq![1, 2], seq![1, 2], |q, k| q == k),
        seq![Some(1), Some(2)]
    );
    assert_eq!(
        xs.first_filtered_by_attention(seq![1, 2], seq![2, 1], |q, k| q == k),
        seq![Some(2), Some(1)]
    );
}
