use husky_toml_utils::read_toml;

use crate::*;

pub struct ToolchainConfig {
    toml: ToolchainToml,
    toolchain: Toolchain,
}

impl ToolchainConfig {
    pub fn toolchain(&self) -> Toolchain {
        self.toolchain
    }
}

#[derive(serde::Deserialize)]
pub struct ToolchainToml {
    #[serde(rename = "toolchain")]
    pub toolchain_section: ToolchainSection,
}

#[derive(serde::Deserialize)]
pub struct ToolchainSection {
    #[serde(rename = "library-path")]
    library_path: PathBuf,
}

/// ad hoc
pub fn toolchain_config(mut dir: &Path, db: &::salsa::Db) -> ToolchainConfig {
    let husky_toolchain_toml_path = loop {
        let husky_toolchain_toml_path = dir.join("husky-toolchain.toml");
        if husky_toolchain_toml_path.exists() {
            break husky_toolchain_toml_path;
        }
        dir = dir.parent().unwrap()
    };
    let toml: ToolchainToml = read_toml(&husky_toolchain_toml_path).unwrap();
    ToolchainConfig {
        toolchain: Toolchain::new(
            db,
            ToolchainData::Local {
                library_path: VirtualPath::try_new(
                    db,
                    dir.join(toml.toolchain_section.library_path.clone()),
                )
                .unwrap(),
            },
        ),
        toml,
    }
}
