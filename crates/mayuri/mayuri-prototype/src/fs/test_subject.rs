use super::*;
use crate::soul::Soul;
use std::path::Path;
use yaml_rust2::Yaml;

#[derive(Debug)]
pub struct MayuriTestSubject {
    path: PathBuf,
    rank: usize,
    soul: Soul,
}

impl MayuriTestSubject {
    pub(super) fn from_file(path: PathBuf) -> impl Iterator<Item = Self> {
        let contents = std::fs::read_to_string(&path).expect("Failed to read the YAML file");

        let docs = yaml_rust2::YamlLoader::load_from_str(&contents).expect("Failed to parse YAML");

        docs.into_iter()
            .enumerate()
            .map(move |(rank, yaml)| Self::from_file_aux(path.clone(), rank, yaml))
    }

    fn from_file_aux(path: PathBuf, rank: usize, yaml: Yaml) -> Self {
        Self {
            path,
            rank,
            soul: Soul::new(&yaml),
        }
    }
}

#[derive(Debug)]
pub struct MayuriTestSubjects {
    subjects: Vec<MayuriTestSubject>,
}

impl MayuriTestSubjects {
    pub(super) fn from_dir(dir: PathBuf) -> Self {
        let mut subjects = Vec::new();

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let path = entry.path();
                        if let Some(extension) = path.extension() {
                            if extension == "yaml" || extension == "yml" {
                                subjects.extend(MayuriTestSubject::from_file(path));
                            }
                        }
                    }
                }
            }
        }

        Self { subjects }
    }
}
