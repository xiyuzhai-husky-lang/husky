use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    packages: Vec<Package>,
    resolve: Resolve,
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    id: String,
    name: String,
    version: String,
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dependency {
    name: String,
    req: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Resolve {
    nodes: Vec<Node>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    id: String,
    dependencies: Vec<String>,
}

fn main() {
    // Run `cargo metadata` and capture the output
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to run cargo metadata");

    // Parse the JSON output
    let metadata: Metadata =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata output");

    // Build a map from package ID to package for quick lookup
    let mut package_map = HashMap::new();
    for package in &metadata.packages {
        package_map.insert(&package.id, package);
    }

    // Build and print the dependency tree
    if let Some(root_package) = metadata.packages.first() {
        println!(
            "Dependency tree for {}:{}",
            root_package.name, root_package.version
        );
        print_dependency_tree(&root_package.id, &metadata.resolve.nodes, &package_map, 0);
    }
}

fn print_dependency_tree(
    package_id: &str,
    nodes: &[Node],
    package_map: &HashMap<&String, &Package>,
    level: usize,
) {
    if let Some(node) = nodes.iter().find(|node| node.id == package_id) {
        if let Some(package) = package_map.get(&node.id) {
            println!(
                "{}- {}:{}",
                "  ".repeat(level),
                package.name,
                package.version
            );
            for dependency in &node.dependencies {
                print_dependency_tree(dependency, nodes, package_map, level + 1);
            }
        }
    }
}
