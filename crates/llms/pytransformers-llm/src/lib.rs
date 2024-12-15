use pyo3::{ffi::c_str, prelude::*};
use std::path::PathBuf;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn python_dir() -> std::io::Result<PathBuf> {
    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(
        cargo_manifest_dir.ends_with("pytransformers-llm"),
        "cargo_manifest_dir: {}",
        cargo_manifest_dir.display()
    );
    let path = cargo_manifest_dir.join("python");
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Python directory not found",
        ));
    }
    Ok(path)
}

#[test]
fn python_dir_works() {
    let python_dir = python_dir().unwrap();
    assert!(python_dir.exists());
    assert!(python_dir.join("lib.py").exists());
}

// try ffi call a function in python/lib.py
fn a() {}

fn call_python_function() -> PyResult<()> {
    Python::with_gil(|py| {
        let py_code = c_str!(include_str!("../python/lib.py"));
        let lib = PyModule::from_code(py, py_code, c_str!("lib.py"), c_str!("lib"))?;

        // Call the Python function
        let result = lib.getattr("example_function")?.call0()?;
        println!("Result from Python: {:?}", result);

        Ok(())
    })
}

#[test]
fn test_python_call() {
    call_python_function().unwrap();
}
