use std::collections::{HashMap, HashSet};

use cargo::core::Summary;
use husky_cargo_utils::metadata::workspace_package_summaries;
use husky_path_utils::HuskyLangDevPaths;
use serde::Serialize;

pub fn husky_lang_packages() -> Vec<PackageSummary> {
    let dev_paths = HuskyLangDevPaths::new();
    workspace_package_summaries(dev_paths.dev_root())
        .into_iter()
        .map(Into::into)
        .collect()
}

#[derive(Debug, Serialize)]
pub struct PackageSummary {
    name: String,
    relative_path: String,
    dependencies: Vec<String>,
}

impl From<Summary> for PackageSummary {
    fn from(summary: Summary) -> Self {
        let url = summary.source_id().url().to_string();
        let url_splits: Vec<&str> = url.split("/husky/").collect();
        assert_eq!(url_splits.len(), 2);
        let relative_path = url_splits[1].to_string();
        Self {
            name: summary.name().to_string(),
            relative_path,
            dependencies: summary
                .dependencies()
                .iter()
                .map(|dep| dep.package_name().to_string())
                .collect(),
        }
    }
}

#[test]
fn husky_lang_packages_works() {
    let packages = husky_lang_packages();
    let dev_paths = HuskyLangDevPaths::new();
    let dir = dev_paths
        .cargo_manifest_dir()
        .map(|p| p.to_owned())
        .unwrap_or("temp".into())
        .join("expect-files");
    std::fs::create_dir_all(&dir).expect("failed_to_create_dir_all");
    let expected = expect_test::expect_file![dir.join("husky_lang_packages.txt")];
    expected.assert_eq(&format!("{packages:#?}"));
}

pub fn husky_lang_jar_packages() -> Vec<JarPackageSummary> {
    let dev_paths = HuskyLangDevPaths::new();
    let package_summaries: Vec<_> = husky_lang_packages()
        .into_iter()
        .filter(|package| {
            (!package.relative_path.starts_with("crates/abstraction"))
                && dev_paths
                    .dev_root()
                    .join(&package.relative_path)
                    .join("src/jar.rs")
                    .exists()
        })
        .collect();
    let package_names: HashSet<String> = package_summaries
        .iter()
        .map(|package| package.name.clone())
        .collect();
    package_summaries
        .into_iter()
        .map(|package| JarPackageSummary::new(package, &package_names))
        .collect()
}

#[derive(Debug, Serialize)]
pub struct JarPackageSummary {
    name: String,
    dependencies: Vec<String>,
}

impl JarPackageSummary {
    fn new(summary: PackageSummary, package_names: &HashSet<String>) -> Self {
        Self {
            name: summary.name,
            dependencies: summary
                .dependencies
                .into_iter()
                .filter(|dep| package_names.contains(dep))
                .collect(),
        }
    }
}

#[test]
fn husky_lang_jar_packages_works() {
    let jar_packages = husky_lang_jar_packages();
    let dev_paths = HuskyLangDevPaths::new();
    let dir = dev_paths
        .cargo_manifest_dir()
        .map(|p| p.to_owned())
        .unwrap_or("temp".into())
        .join("expect-files");
    std::fs::create_dir_all(&dir).expect("failed_to_create_dir_all");
    let expected = expect_test::expect_file![dir.join("husky_lang_jar_packages.txt")];
    expected.assert_eq(&format!("{jar_packages:#?}"));
}

struct JarTreeBuilder<'a> {
    jar_package_summaries: &'a [JarPackageSummary],
    dependencies_table: HashMap<String, HashSet<String>>,
}

impl<'a> JarTreeBuilder<'a> {
    fn new(jar_package_summaries: &'a [JarPackageSummary]) -> Self {
        Self {
            jar_package_summaries,
            dependencies_table: Default::default(),
        }
    }
}

impl<'a> JarTreeBuilder<'a> {
    fn build_all(&mut self) {
        for summary in self.jar_package_summaries {
            self.build(&summary);
        }
    }

    fn build(&mut self, summary: &'a JarPackageSummary) {
        if self.dependencies_table.contains_key(&summary.name) {
            return;
        }
        println!("building {}", &summary.name);
        let mut dependencies: HashSet<String> = Default::default();
        for dep in &summary.dependencies {
            if dep != &summary.name {
                dependencies.insert(dep.clone());
                let dep_summary = self
                    .jar_package_summaries
                    .iter()
                    .find(|summary| &summary.name == dep)
                    .unwrap();
                self.build(dep_summary);
                dependencies.extend(self.dependencies_table[dep].clone())
            }
        }
        self.dependencies_table
            .insert(summary.name.clone(), dependencies);
    }

    fn finish(self) -> HashMap<String, HashSet<String>> {
        self.dependencies_table
    }
}

pub fn husky_lang_jar_tree() -> HashMap<String, HashSet<String>> {
    let jar_packages = husky_lang_jar_packages();
    let mut builder = JarTreeBuilder::new(&jar_packages);
    builder.build_all();
    builder.finish()
}

#[test]
fn husky_lang_jar_tree_works() {
    let jar_tree = husky_lang_jar_tree();
    let dev_paths = HuskyLangDevPaths::new();
    let dir = dev_paths
        .cargo_manifest_dir()
        .map(|p| p.to_owned())
        .unwrap_or("temp".into())
        .join("expect-files");
    std::fs::create_dir_all(&dir).expect("failed_to_create_dir_all");
    let expected = expect_test::expect_file![dir.join("husky_lang_jar_tree.txt")];
    expected.assert_eq(&format!("{jar_tree:#?}"));
}

pub fn husky_lang_jar_tree_trimmed() -> HashMap<String, HashSet<String>> {
    let jar_tree = husky_lang_jar_tree();
    jar_tree
        .iter()
        .map(|(name, deps)| {
            let mut deps_trimmed = deps.clone();
            for dep in deps {
                for dep_dep in &jar_tree[dep] {
                    deps_trimmed.remove(dep_dep);
                }
            }
            (name.clone(), deps_trimmed)
        })
        .collect()
}

#[test]
fn husky_lang_jar_tree_trimmed_works() {
    let jar_tree_trimmed = husky_lang_jar_tree_trimmed();
    let dev_paths = HuskyLangDevPaths::new();
    let dir = dev_paths
        .cargo_manifest_dir()
        .map(|p| p.to_owned())
        .unwrap_or("temp".into())
        .join("expect-files");
    std::fs::create_dir_all(&dir).expect("failed_to_create_dir_all");
    let expected = expect_test::expect_file![dir.join("husky_lang_jar_tree_trimmed.txt")];
    expected.assert_eq(&format!("{jar_tree_trimmed:#?}"));
}
