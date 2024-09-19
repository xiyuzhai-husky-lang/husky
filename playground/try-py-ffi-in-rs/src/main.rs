use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use std::env;
use std::path::PathBuf;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();

    println!("Hello, world!");

    Python::with_gil(|py| {
        let sys = py.import_bound("sys")?;
        let path: Bound<PyAny> = sys.getattr("path")?;

        // Get the project root path using CARGO_MANIFEST_DIR
        let project_root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

        // Add the project root to Python's sys.path
        path.call_method1("append", (project_root,))?;

        let example = py.import_bound("example")?;

        // Call the greet function
        let name = "Rust Developer";
        let result: String = example.getattr("greet")?.call1((name,))?.extract()?;
        println!("{}", result);

        // Call the add_numbers function
        let a = 5;
        let b = 7;
        let sum: i32 = example.getattr("add_numbers")?.call1((a, b))?.extract()?;
        println!("{} + {} = {}", a, b, sum);

        Ok(())
    })
}
