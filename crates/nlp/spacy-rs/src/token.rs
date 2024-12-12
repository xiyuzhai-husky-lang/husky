use crate::*;
use pyo3::types::{PyInt, PyString};

pub struct SpacyToken {
    pub text: String,
    pub lemma: String,
    pub pos: String,
    pub tag: String,
    pub dep: String,
    pub head: usize,
}

impl SpacyToken {
    pub fn from_python(token: &Bound<PyAny>) -> Self {
        Self {
            text: token.downcast::<PyString>().unwrap().to_string(),
            lemma: token.downcast::<PyString>().unwrap().to_string(),
            pos: token.downcast::<PyString>().unwrap().to_string(),
            tag: token.downcast::<PyString>().unwrap().to_string(),
            dep: token.downcast::<PyString>().unwrap().to_string(),
            head: token.downcast::<PyInt>().unwrap().extract().unwrap(),
        }
    }
}
