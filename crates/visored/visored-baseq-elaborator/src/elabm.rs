use crate::{elaborator::VdBsqElaboratorInner, Mhr};

type E<'db, 'sess> = VdBsqElaboratorInner<'db, 'sess>;
type R<'sess> = Mhr<'sess>;

pub trait ElabM<'db, 'sess, T>
where
    'db: 'sess,
{
    fn eval(
        self,
        engine: &mut E<'db, 'sess>,
        f: &dyn Fn(&mut E<'db, 'sess>, T) -> R<'sess>,
    ) -> R<'sess>;
    fn map<S>(self, f: impl Fn(&mut E<'db, 'sess>, T) -> S) -> impl ElabM<'db, 'sess, S>
    where
        Self: Sized,
    {
        move |engine: &mut E<'db, 'sess>,
              g: &dyn Fn(&mut E<'db, 'sess>, S) -> R<'sess>|
              -> R<'sess> {
            self.eval(engine, &|engine: &mut E<'db, 'sess>, t| {
                let s = f(engine, t);
                g(engine, s)
            })
        }
    }
    fn bind<S, M>(self, f: impl Fn(&mut E<'db, 'sess>, T) -> M) -> impl ElabM<'db, 'sess, S>
    where
        Self: Sized,
        M: ElabM<'db, 'sess, S>,
    {
        move |engine: &mut E<'db, 'sess>,
              g: &dyn Fn(&mut E<'db, 'sess>, S) -> R<'sess>|
              -> R<'sess> {
            self.eval(engine, &|engine: &mut E<'db, 'sess>, t| {
                let m = f(engine, t);
                m.eval(engine, g)
            })
        }
    }
}

impl<'db, 'sess, T, F> ElabM<'db, 'sess, T> for F
where
    'db: 'sess,
    F: FnOnce(&mut E<'db, 'sess>, &dyn Fn(&mut E<'db, 'sess>, T) -> R<'sess>) -> R<'sess>,
{
    fn eval(
        self,
        engine: &mut E<'db, 'sess>,
        f: &dyn Fn(&mut E<'db, 'sess>, T) -> R<'sess>,
    ) -> R<'sess> {
        self(engine, f)
    }
}

pub(crate) struct Pure<T>(pub T);

impl<'db, 'sess, T> ElabM<'db, 'sess, T> for Pure<T>
where
    'db: 'sess,
{
    fn eval(
        self,
        engine: &mut E<'db, 'sess>,
        f: &dyn Fn(&mut E<'db, 'sess>, T) -> R<'sess>,
    ) -> R<'sess> {
        f(engine, self.0)
    }
}

impl<'db, 'sess, T> ElabM<'db, 'sess, T> for !
where
    'db: 'sess,
{
    fn eval(
        self,
        engine: &mut E<'db, 'sess>,
        f: &dyn Fn(&mut E<'db, 'sess>, T) -> R<'sess>,
    ) -> R<'sess> {
        unreachable!()
    }
}

impl<'db, 'sess, T> ElabM<'db, 'sess, T> for ()
where
    'db: 'sess,
{
    #[track_caller]
    fn eval(
        self,
        engine: &mut E<'db, 'sess>,
        f: &dyn Fn(&mut E<'db, 'sess>, T) -> R<'sess>,
    ) -> R<'sess> {
        todo!()
    }
}
