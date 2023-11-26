use enum_index::full_map::EnumFullVecMap;

use super::routes::Routes;
use crate::test_utils::{HasTestJarIndex, TestJarIndex};

pub trait Jar<'db>: Sized {
    fn initialize(&mut self, routes: &mut Routes);
}

pub struct Jars {
    map: EnumFullVecMap<TestJarIndex, Option<Box<dyn std::any::Any + Sync + Send>>>,
}

impl Jars {
    fn jar<Jar>(&self) -> &Jar
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

    fn jar_mut<Jar>(&mut self) -> &mut Jar
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
