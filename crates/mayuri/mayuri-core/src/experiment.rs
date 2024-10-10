pub mod path;

use self::path::ExperimentPath;
use crate::{
    src::{MayuriSrc, MayuriSrcFile},
    *,
};
use config::nemu::NemuConfig;
use husky_sha_utils::{Sha256Output, Sha512Output, ShaHash};
use husky_yaml_utils::ordered::OrderedYaml;
use makefile::MayuriMakefileExtracted;
use vec_like::ordered_vec_map::OrderedVecPairMap;
use yaml_rust2::Yaml;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Experiment {
    path: ExperimentPath,
    path_sha256: Sha256Output,
    /// maps destination paths to files
    src_files: ExperimentSrcFiles,
    src_files_sha512: Sha512Output,
    config: OrderedYaml,
    makefile: MayuriMakefileExtracted,
}

pub type ExperimentSrcFiles = OrderedVecPairMap<String, MayuriSrcFile>;

impl Experiment {
    pub(super) fn new(
        yaml: &Yaml,
        src: &MayuriSrc,
        makefile: MayuriMakefileExtracted,
        nemu_config: &NemuConfig,
    ) -> Self {
        let path = ExperimentPath::new(yaml, nemu_config);
        let src_files: ExperimentSrcFiles = nemu_config
            .src_paths()
            .iter()
            .map(|src_path| (src_path.path().to_string(), src[src_path.path()].clone()))
            .chain(
                yaml["src"]
                    .as_hash()
                    .expect("expected hash")
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.as_str()
                                .expect("invalid yaml, expected string")
                                .to_string(),
                            src[v.as_str().expect("invalid, expected string")].clone(),
                        )
                    }),
            )
            .collect();
        Self {
            path_sha256: path.sha256(),
            src_files_sha512: src_files.sha512(),
            path,
            src_files,
            makefile,
            config: OrderedYaml::new(&yaml["config"]),
        }
    }
}

impl Experiment {
    fn run_local(&self) {
        todo!()
    }

    /// Copy code from origin to destination
    fn prepare_local(&self) -> std::io::Result<()> {
        use std::fs::{self, File};
        use std::io::Write;
        use std::path::Path;

        for (dest, src_file) in &self.src_files {
            let dest_path = Path::new(dest);

            // Create parent directories if they don't exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Read the content from the origin file
            let content = &src_file.content;

            // Write the content to the destination file
            let mut file = File::create(dest_path)?;
            file.write_all(content.as_bytes())?;
        }

        Ok(())
    }
}
