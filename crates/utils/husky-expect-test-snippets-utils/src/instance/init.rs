use super::*;

impl ExpectInstance {
    pub(super) fn init(&mut self) -> DesIoResult<()> {
        let inputs = read_to_value(&self.test_inputs_json_path())?;
        let results = read_to_value(&self.test_results_json_path()).ok();
        self.init_entries(inputs, results)
    }

    fn init_entries(
        &mut self,
        inputs: Vec<String>,
        mut results: Option<Vec<ExpectEntry>>,
    ) -> DesIoResult<()> {
        check_no_repetition(&inputs);
        self.entries = inputs
            .into_iter()
            .map(|input| ExpectEntry {
                output: if let Some(results) = results.as_mut() {
                    match results.iter_mut().find(|entry| entry.input == input) {
                        Some(entry) => std::mem::take(&mut entry.output),
                        None => None,
                    }
                } else {
                    None
                },
                input,
            })
            .collect();
        Ok(())
    }
}

fn read_to_value<T>(path: &Path) -> DesIoResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    Ok(serde_json::from_str(&read_to_string(path).map_err(
        |e| DesIoError::IO {
            e,
            path: path.to_owned(),
        },
    )?)?)
}

fn check_no_repetition(inputs: &[String]) {
    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            if inputs[i] == inputs[j] {
                todo!()
            }
        }
    }
}
