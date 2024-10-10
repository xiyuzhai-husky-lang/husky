//! Nemu is like Cargo, the manager of things
use super::*;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct NemuConfig {
    #[serde(rename = "src")]
    src_paths: Vec<SrcPath>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SrcPath {
    path: String,
}

impl SrcPath {
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl NemuConfig {
    pub fn src_paths(&self) -> &[SrcPath] {
        &self.src_paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn config_works() {
        let toml_str = r#"
                [[src]]
                path = "main.py"
                [[src]]
                path = "eval.py"
            "#;

        let config: NemuConfig = toml::from_str(toml_str).unwrap();

        let expected_config = NemuConfig {
            src_paths: vec![
                SrcPath {
                    path: "main.py".to_string(),
                },
                SrcPath {
                    path: "eval.py".to_string(),
                },
            ],
        };

        assert_eq!(config, expected_config);
    }
}
