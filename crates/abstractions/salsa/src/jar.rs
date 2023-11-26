use std::mem::MaybeUninit;

use enum_index::full_map::EnumFullVecMap;

use super::routes::Routes;
use crate::test_utils::{HasTestJarIndex, TestJarIndex};

pub trait Jar<'db>: Sized {
    fn initialize(&mut self, routes: &mut Routes);
}

#[derive(Default)]
pub struct Jars {
    map: EnumFullVecMap<TestJarIndex, Option<Box<dyn std::any::Any + Sync + Send>>>,
}

impl Jars {
    pub fn initialize_jar<Jar>(&mut self, routes: &mut Routes)
    where
        Jar: for<'db> crate::jar::Jar<'db> + HasTestJarIndex + Send + Sync + 'static,
    {
        let mut jar_maybe_uninitialized: MaybeUninit<Jar> = MaybeUninit::uninit();
        let jar: &mut Jar = unsafe { std::mem::transmute(&mut jar_maybe_uninitialized) };
        Jar::initialize(jar, routes);
        let index = <Jar as HasTestJarIndex>::TEST_JAR_INDEX;
        debug_assert!(self.map[index].is_none());
        self.map[index] =
            Some(unsafe { std::mem::transmute::<_, Box<Jar>>(Box::new(jar_maybe_uninitialized)) })
    }

    pub fn jar<Jar>(&self) -> &Jar
    where
        Jar: HasTestJarIndex + 'static,
    {
        let any: &Box<dyn std::any::Any + Send + Sync + 'static> = self.map
            [<Jar as HasTestJarIndex>::TEST_JAR_INDEX]
            .as_ref()
            .expect("should be initialized");
        let any: &(dyn std::any::Any + Send + Sync + 'static) = &**any;
        any.downcast_ref().expect("should be the right type")
    }

    pub fn jar_mut<Jar>(&mut self) -> &mut Jar
    where
        Jar: HasTestJarIndex + 'static,
    {
        let any: &mut Box<dyn std::any::Any + Send + Sync + 'static> = self.map
            [<Jar as HasTestJarIndex>::TEST_JAR_INDEX]
            .as_mut()
            .expect("should be initialized");
        let any: &mut (dyn std::any::Any + Send + Sync + 'static) = &mut **any;
        any.downcast_mut().expect("should be the right type")
    }
}
