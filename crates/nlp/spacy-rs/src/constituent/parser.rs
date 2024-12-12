use pyo3::PyObject;
use std::sync::OnceLock;

pub struct SpacyConstituentParser {
    pyobject: OnceLock<PyObject>,
}

impl SpacyConstituentParser {
    pub fn new() -> Self {
        Self {
            pyobject: OnceLock::new(),
        }
    }
}

impl SpacyConstituentParser {
    pub fn parse(&self, sentence: &str) -> String {
        todo!()
    }
}
