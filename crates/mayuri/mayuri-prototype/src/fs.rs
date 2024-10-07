use husky_config_utils::IsConfig;
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::PathBuf;

impl MayuriFs {
    pub fn new(root: PathBuf) -> io::Result<Self> {
        let mayuri_config = MayuriConfig::read_from_toml_file(&root.join("Mayuri.toml"))?;
        let nemu_config = NemuConfig::read_from_toml_file(&root.join("Nemu.toml"))?;

        Ok(MayuriFs {
            root,
            mayuri_config,
            nemu_config,
        })
    }
}

#[derive(Deserialize)]
pub struct MayuriConfig {
    // Add fields as needed
}

#[derive(Deserialize)]
pub struct NemuConfig {
    // Add fields as needed
}

pub struct MayuriFs {
    root: PathBuf,
    /// read from `<root>/Mayuri.toml`
    mayuri_config: MayuriConfig,
    /// read from `<root>/Nemu.toml`
    nemu_config: NemuConfig,
}

#[test]
fn mayuri_fs_works() {
    use husky_path_utils::HuskyLangDevPaths;
    let dev_paths = HuskyLangDevPaths::new();
    let experiments_dir = dev_paths.experiments_dir();
    let mayuri_prototype_dir = experiments_dir.join("mayuri-prototype");
    let fs = MayuriFs::new(mayuri_prototype_dir).unwrap();
}
