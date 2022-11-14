use crate::*;
use thiserror::Error;

pub(crate) struct ExpectContent {
    test_data_path: PathBuf,
    entries: Vec<ExpectEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct ExpectEntry {
    pub(crate) input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output: Option<String>,
}

impl ExpectContent {
    pub(crate) fn extract_from_file(test_data_path: PathBuf) -> Result<Self, DesIoError> {
        if !test_data_path.exists() || !test_data_path.is_file() {
            return Err(DesIoError::NotValidFile(test_data_path.to_owned()));
        }
        let content = read_to_string(&test_data_path)?;
        Ok(Self {
            test_data_path,
            entries: serde_json::from_str(&content)?,
        })
    }

    pub(crate) fn save_to_file(&self) {
        diff_write(
            &self.test_data_path,
            &serde_json::to_string_pretty(&self.entries).unwrap(),
            true,
        );
    }

    fn entries<'a>(&'a mut self) -> impl Iterator<Item = &mut ExpectEntry> + 'a {
        self.entries.iter_mut()
    }

    pub(crate) fn update(&mut self, f: &impl Fn(&str) -> String) {
        let result = self.try_update(f);
        self.save_to_file();
        match result {
            Ok(_) => (),
            Err(e) => {
                panic!("{e}")
            }
        }
    }

    fn try_update(&mut self, f: &impl Fn(&str) -> String) -> UpdateExpectResult<()> {
        for expect in self.entries() {
            let output = f(&expect.input);
            if expect.output.as_ref() != Some(&output) {
                match ask_is_input_output_okay(&expect.input, &output) {
                    true => expect.output = Some(output),
                    false => return Err(UpdateExpectError::OutputNotAccepted),
                }
            }
        }
        Ok(())
    }

    pub(crate) fn verify(&self, f: &impl Fn(&str) -> String) {
        for expect in self.entries.iter() {
            let output = f(&expect.input);
            assert_eq!(expect.output, Some(output))
        }
    }
}

#[derive(Debug, Error)]
pub enum UpdateExpectError {
    #[error("output not accepted")]
    OutputNotAccepted,
}

pub type UpdateExpectResult<T> = Result<T, UpdateExpectError>;
