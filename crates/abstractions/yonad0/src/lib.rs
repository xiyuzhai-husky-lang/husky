use std::marker::PhantomData;

pub struct Yonad<'a, E, R, T> {
    inner: Box<dyn Fn(&mut E, &dyn Fn(&mut E, &T) -> R) -> R + 'a>,
}

impl<'a, E, R, T> Yonad<'a, E, R, T> {
    pub fn new(inner: impl Fn(&mut E, &dyn Fn(&mut E, &T) -> R) -> R + 'a) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<'a, E, R, T> Yonad<'a, E, R, T>
where
    E: 'a,
    R: 'a,
    T: 'a,
{
    pub fn bind<S>(self, g: impl Fn(&T) -> Yonad<'a, E, R, S> + 'a) -> Yonad<'a, E, R, S> {
        Yonad::new(move |env, f: &dyn Fn(&mut E, &S) -> R| {
            (self.inner)(env, &|env, t| (g(t).inner)(env, f))
        })
    }
}
