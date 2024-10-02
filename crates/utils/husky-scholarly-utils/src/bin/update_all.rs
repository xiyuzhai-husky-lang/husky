use husky_scholarly_utils::ScholarlyRuntime;
use pyo3::types::PyAnyMethods;
use pyo3::Python;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Print Python version
        let sys = py.import_bound("sys")?;
        let version: String = sys.getattr("version")?.extract()?;
        println!("Python version: {}", version);

        // Print Python path
        let path: Vec<String> = sys.getattr("path")?.extract()?;
        println!("Python path:");
        for p in path {
            println!("  {}", p);
        }

        match py.import_bound("scholarly") {
            Ok(_) => println!("scholarly module imported successfully"),
            Err(e) => println!("Error importing scholarly: {:?}", e),
        }
        Ok::<_, Box<dyn std::error::Error>>(())
    })?;
    let scholarly_runtime = ScholarlyRuntime::new()?;

    let query = "attention is all you need";
    // Perform the search
    let results = scholarly_runtime.search_pubs(query)?;

    // Print the results
    println!("{}", results);

    Ok(())
}
