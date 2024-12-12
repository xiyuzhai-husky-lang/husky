use pyo3::prelude::*;

pub fn py_module_from_path(path: &str) -> PyResult<Py<PyModule>> {
    Python::with_gil(|py| {
        // Get the module name from the last component of the path
        let module_name = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid module path")
            })?;

        // Add the parent directory to Python's sys.path
        let dir_path = std::path::Path::new(path).parent().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid module path")
        })?;

        let sys = py.import_bound("sys")?;
        sys.getattr("path")?
            .call_method1("append", (dir_path.to_str().unwrap(),))?;

        // Import the module by its name
        Ok(py.import_bound(module_name)?.unbind())
    })
}

#[test]
fn py_module_from_path_works() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    assert!(py_module_from_path(&(format!("{}/python/lib.py", manifest_dir))).is_ok());
    assert!(py_module_from_path(&(format!("{}/python/strange.py", manifest_dir))).is_ok());
    assert!(py_module_from_path(&(format!("{}/python/diablo2/poe2.py", manifest_dir))).is_ok());
}
