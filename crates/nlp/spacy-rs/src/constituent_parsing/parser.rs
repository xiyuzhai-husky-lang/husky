use super::*;
use std::sync::OnceLock;

pub struct SpacyConstituentParser {
    module: Py<PyModule>,
    parser_pyobject: OnceLock<PyObject>,
}

impl SpacyConstituentParser {
    pub fn new() -> Self {
        Self {
            module: Python::with_gil(|py| {
                PyModule::import(py, "constituency_parser")
                    .unwrap()
                    .unbind()
            }),
            parser_pyobject: OnceLock::new(),
        }
    }
}

impl SpacyConstituentParser {
    pub fn parse(&self, sentence: &str) -> String {
        Python::with_gil(|py| {
            let parser = self.get_parser_pyobject(py);
            todo!()
        });
        todo!()
    }

    fn get_parser_pyobject(&self, py: Python<'_>) -> Bound<PyAny> {
        self.parser_pyobject
            .get_or_init(|| Python::with_gil(|py| todo!()));
        todo!()
    }
}
