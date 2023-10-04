use std::sync::Arc;

pub trait Find<T> {
    fn find_last(&self, f: impl Fn(&T) -> bool) -> Option<&T>;
}

impl<T> Find<T> for () {
    fn find_last(&self, _f: impl Fn(&T) -> bool) -> Option<&T> {
        None
    }
}

impl<T> Find<T> for Vec<T> {
    fn find_last(&self, f: impl Fn(&T) -> bool) -> Option<&T> {
        self.iter().rev().find(|item| f(*item))
    }
}

impl<T> Find<T> for [T] {
    fn find_last(&self, f: impl Fn(&T) -> bool) -> Option<&T> {
        self.iter().rev().find(|item| f(*item))
    }
}
impl<'a, T, V> Find<T> for &'a V
where
    V: Find<T> + ?Sized,
{
    fn find_last(&self, f: impl Fn(&T) -> bool) -> Option<&T> {
        V::find_last(self, f)
    }
}

impl<T, V> Find<T> for Arc<V>
where
    V: Find<T>,
{
    fn find_last(&self, f: impl Fn(&T) -> bool) -> Option<&T> {
        V::find_last(&self, f)
    }
}

impl<T, V> Find<T> for Option<V>
where
    V: Find<T>,
{
    fn find_last(&self, f: impl Fn(&T) -> bool) -> Option<&T> {
        let v = self.as_ref()?;
        V::find_last(v, f)
    }
}

#[test]
fn find_vec_works() {
    let vs = vec![1, 2, 3];
    assert_eq!(vs.find_last(|x| *x == 0), None);
    assert_eq!(vs.find_last(|x| *x == 1), Some(&1));
    assert_eq!(vs.find_last(|x| *x == 2), Some(&2));
    assert_eq!(vs.find_last(|x| *x == 3), Some(&3));
}

#[test]
fn find_arc_vec_works() {
    let vs = Arc::new(vec![1, 2, 3]);
    assert_eq!(vs.find_last(|x| *x == 0), None);
    assert_eq!(vs.find_last(|x| *x == 1), Some(&1));
    assert_eq!(vs.find_last(|x| *x == 2), Some(&2));
    assert_eq!(vs.find_last(|x| *x == 3), Some(&3));
}

#[test]
fn find_opt_arc_vec_works() {
    let some_vs = Some(Arc::new(vec![1, 2, 3]));
    assert_eq!(some_vs.find_last(|x| *x == 0), None);
    assert_eq!(some_vs.find_last(|x| *x == 1), Some(&1));
    assert_eq!(some_vs.find_last(|x| *x == 2), Some(&2));
    assert_eq!(some_vs.find_last(|x| *x == 3), Some(&3));
    let none_vs: Option<Arc<Vec<i32>>> = None;
    assert_eq!(none_vs.find_last(|x| *x == 0), None);
    assert_eq!(none_vs.find_last(|x| *x == 1), None);
    assert_eq!(none_vs.find_last(|x| *x == 2), None);
    assert_eq!(none_vs.find_last(|x| *x == 3), None);
}
