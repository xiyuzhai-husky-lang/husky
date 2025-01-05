pub mod jar;
pub mod jar0;

use sealed::sealed;
use std::pin::Pin;

pub trait IsNote: 'static {
    type Jar: IsNoteJar;
}

#[sealed]
pub trait IsNoteJar: IsNoteJarDyn + Default + 'static {}

#[sealed]
pub trait IsNoteJarDyn: std::any::Any + Send + Sync + 'static {}

pub struct NoteJarDyn(Pin<Box<dyn IsNoteJarDyn>>);

impl NoteJarDyn {
    pub fn new<N: IsNote>() -> Self {
        Self(Box::pin(N::Jar::default()))
    }

    pub fn downcast<J: IsNoteJarDyn>(&self) -> &J {
        (&*self.0 as &dyn std::any::Any)
            .downcast_ref::<J>()
            .unwrap()
    }
}
