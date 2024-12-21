mod subprocess;

use self::subprocess::SglangLlmSubprocess;

impl SglangLlmSubprocess {
    pub fn generate_batch(&self, prompts: Vec<String>) -> Vec<String> {
        // Example: Send numbers to container program for addition

        let mut subprocess = SglangLlmSubprocess::new();
        subprocess.write_line(serde_json::to_string(&prompts).unwrap());

        // Read result from container's stdout
        let line = subprocess.read_line().unwrap();
        // Only print lines that contain numeric results
        serde_json::from_str(&line).unwrap()
    }
}

#[test]
fn add_works() {
    let subprocess = SglangLlmSubprocess::new();
    println!(
        "{:?}",
        subprocess.generate_batch(vec![
            "Why Soifon hates Urahara?".to_string(),
            "How is Ywach defeated?".to_string()
        ])
    );
}
