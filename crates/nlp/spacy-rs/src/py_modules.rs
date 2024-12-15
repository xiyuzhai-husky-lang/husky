use pyo3::{ffi::c_str, types::PyModule, Bound, Py, Python};
use std::{ffi::CStr, sync::OnceLock};

static CONSTITUENT_PARSING_PY_MODULE: OnceLock<force_send_sync::SendSync<Py<PyModule>>> =
    OnceLock::new();

pub fn get_constituent_parsing_module<'py>(py: Python<'py>) -> &Bound<'py, PyModule> {
    CONSTITUENT_PARSING_PY_MODULE
        .get_or_init(|| {
            let py_code = c_str!(include_str!("../python/constituent_parsing.py"));
            unsafe {
                force_send_sync::SendSync::new(
                    PyModule::from_code(
                        py,
                        py_code,
                        c_str!("constituent_parsing.py"),
                        c_str!("constituent_parsing"),
                    )
                    .unwrap()
                    .unbind(),
                )
            }
        })
        .bind(py)
}
