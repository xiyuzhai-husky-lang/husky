use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct MayuriConfig {}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SrcConfig {
    pub path: PathBuf,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct NemuConfig {
    src: Vec<NemuSrcFragment>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct NemuSrcFragment {
    path: String,
}

#[cfg(test)]
use toml;

#[test]
fn mayuri_config_works() {
    let toml_str = r#"
    "#;

    assert_eq!(MayuriConfig {}, toml::from_str(toml_str).unwrap());
}

#[test]
fn nemu_config_works() {
    let toml_str = r#"
        [[src]]
        path = "main.py"
        [[src]]
        path = "eval.py"
    "#;

    let config: NemuConfig = toml::from_str(toml_str).unwrap();

    let expected_config = NemuConfig {
        src: vec![
            NemuSrcFragment {
                path: "main.py".to_string(),
            },
            NemuSrcFragment {
                path: "eval.py".to_string(),
            },
        ],
    };

    assert_eq!(config, expected_config);
}
