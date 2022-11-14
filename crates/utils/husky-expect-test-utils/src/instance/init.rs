use super::*;
use std::collections::HashMap;

impl ExpectInstance {
    pub(super) fn init(&mut self) -> DesIoResult<()> {
        let inputs = read_to_value(&self.test_path_stem.join("test-inputs.json"))?;
        let results = read_to_value(&self.test_path_stem.join("test-results.json")).ok();
        self.init_entries(inputs, results)
    }

    fn init_entries(
        &mut self,
        inputs: Vec<String>,
        results: Option<Vec<ExpectEntry>>,
    ) -> DesIoResult<()> {
        todo!()
    }
}

fn read_to_value<T>(path: &Path) -> DesIoResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    Ok(serde_json::from_str(&read_to_string(path)?)?)
}
