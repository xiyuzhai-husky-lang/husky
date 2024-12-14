use crate::*;
use std::path::Path;

pub fn py_module_from_path(path: &Path) -> PyResult<Py<PyModule>> {
    Python::with_gil(|py| {
        // Get the module name from the last component of the path
        let module_name = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid module path")
        })?;

        // Add the parent directory to Python's sys.path
        let dir_path = path.parent().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid module path")
        })?;

        if !dir_path.exists() {
            use husky_print_utils::p;
            p!(dir_path);
            todo!()
        }
        let sys = py.import_bound("sys")?;
        sys.getattr("path")?
            .call_method1("append", (dir_path.to_str().unwrap(),))?;

        // Import the module by its name
        let pymodule = match py.import_bound(module_name) {
            Ok(pymodule) => pymodule,
            Err(err) => todo!("error importing module: {err}"),
        };
        Ok(pymodule.unbind())
    })
}

#[test]
pub(crate) fn py_module_from_path_works() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(py_module_from_path(&manifest_dir.join("python/lib.py")).is_ok());
    assert!(py_module_from_path(&manifest_dir.join("python/strange.py")).is_ok());
    assert!(py_module_from_path(&manifest_dir.join("python/diablo2/poe2.py")).is_ok());
}
