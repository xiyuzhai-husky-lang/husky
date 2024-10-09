use super::*;
use crate::{experiment::Experiment, src::MayuriSrc};
use std::path::Path;
use yaml_rust2::Yaml;

#[derive(Debug)]
pub struct MayuriTestSubject {
    path: PathBuf,
    rank: usize,
    experiment: Experiment,
}

impl MayuriTestSubject {
    pub(super) fn from_file(path: PathBuf, src: &MayuriSrc) -> impl Iterator<Item = Self> + '_ {
        let contents = std::fs::read_to_string(&path).expect("Failed to read the YAML file");

        let docs = yaml_rust2::YamlLoader::load_from_str(&contents).expect("Failed to parse YAML");
        docs.into_iter()
            .enumerate()
            .map(move |(rank, yaml)| Self::from_file_aux(path.clone(), rank, yaml, src))
    }

    fn from_file_aux(path: PathBuf, rank: usize, yaml: Yaml, src: &MayuriSrc) -> Self {
        Self {
            path,
            rank,
            experiment: Experiment::new(&yaml, src),
        }
    }
}

#[derive(Debug)]
pub struct MayuriTestSubjects {
    subjects: Vec<MayuriTestSubject>,
}

impl MayuriTestSubjects {
    pub(super) fn from_dir(dir: PathBuf, src: &MayuriSrc) -> Self {
        let mut subjects = Vec::new();

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let path = entry.path();
                        if let Some(extension) = path.extension() {
                            if extension == "yaml" || extension == "yml" {
                                subjects.extend(MayuriTestSubject::from_file(path, src));
                            }
                        }
                    }
                }
            }
        }

        Self { subjects }
    }
}
