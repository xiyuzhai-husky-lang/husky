use super::*;

pub fn mapm_collect<'a, 'db, 'sess, S, A, I, MA, R>(
    iter: I,
    f: &'a impl Fn(&mut Elr<'db, 'sess>, I::Item) -> MA,
) -> impl ElabM<'db, 'sess, S> + 'a
where
    'db: 'sess,
    S: Default + Extend<A> + Clone + 'a,
    I: IntoIterator + 'a,
    I::IntoIter: Clone,
    MA: ElabM<'db, 'sess, A> + 'a,
{
    miracle::foldm::mapm_collect(iter.into_iter(), move |engine, item| {
        move |engine, heuristic| f(engine, item).eval(engine, heuristic)
    })
}
