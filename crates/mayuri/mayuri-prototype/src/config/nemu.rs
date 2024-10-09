//! Nemu is like Cargo, the manager of things
use super::*;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct NemuConfig {
    src: Vec<SrcFragment>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SrcFragment {
    path: String,
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
            src: vec![
                SrcFragment {
                    path: "main.py".to_string(),
                },
                SrcFragment {
                    path: "eval.py".to_string(),
                },
            ],
        };

        assert_eq!(config, expected_config);
    }
}
