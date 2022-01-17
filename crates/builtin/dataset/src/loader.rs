use std::any::Any;

pub trait LoadSample {
    fn load(&mut self, idx: usize) -> &dyn Any;
}

pub type SampleLoader<'a> = Box<dyn LoadSample + 'a>;
