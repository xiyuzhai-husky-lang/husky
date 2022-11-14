use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct Entry {
    pub(crate) input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output: Option<String>,
}

impl Entry {
    pub(crate) fn extract_from_file(path: &Path) -> Result<Vec<Self>, DesIoError> {
        if !path.exists() || !path.is_file() {
            return Err(DesIoError::NotValidFile(path.to_owned()));
        }
        let content = read_to_string(&path)?;
        Ok(serde_json::from_str(&content)?)
    }
}
