use crate::*;
use config::nemu::NemuConfig;
use experiment::Experiment;
use src::MayuriSrc;
use std::path::PathBuf;
use yaml_rust2::Yaml;

#[derive(Debug)]
pub struct MayuriTest {
    path: PathBuf,
    rank: usize,
    experiment: Experiment,
}

impl MayuriTest {
    pub(super) fn from_file<'a>(
        path: PathBuf,
        src: &'a MayuriSrc,
        nemu_config: &'a NemuConfig,
    ) -> impl Iterator<Item = Self> + 'a {
        let contents = std::fs::read_to_string(&path).expect("Failed to read the YAML file");

        let docs = yaml_rust2::YamlLoader::load_from_str(&contents).expect("Failed to parse YAML");
        docs.into_iter().enumerate().map(move |(rank, yaml)| {
            Self::from_file_aux(path.clone(), rank, yaml, src, nemu_config)
        })
    }

    fn from_file_aux(
        path: PathBuf,
        rank: usize,
        yaml: Yaml,
        src: &MayuriSrc,
        nemu_config: &NemuConfig,
    ) -> Self {
        Self {
            path,
            rank,
            experiment: Experiment::new(&yaml, src, nemu_config),
        }
    }
}

#[derive(Debug)]
pub struct MayuriTests {
    tests: Vec<MayuriTest>,
}

impl MayuriTests {
    pub(crate) fn from_dir(dir: PathBuf, src: &MayuriSrc, nemu_config: &NemuConfig) -> MayuriTests {
        let mut subjects = Vec::new();

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let path = entry.path();
                        if let Some(extension) = path.extension() {
                            if extension == "yaml" || extension == "yml" {
                                subjects.extend(MayuriTest::from_file(path, src, nemu_config));
                            }
                        }
                    }
                }
            }
        }

        Self { tests: subjects }
    }

    pub(crate) fn run_all(&self) {
        self.tests.iter().for_each(|test| test.run());
    }
}

impl MayuriTest {
    fn run(&self) {
        todo!()
    }
}
