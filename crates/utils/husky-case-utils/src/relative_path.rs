use crate::*;
use ::relative_path::RelativePathBuf;
use std::path::{Path, PathBuf};

pub fn convert_path_case(path: impl AsRef<Path>, case: Case) -> PathBuf {
    let converted = path.as_ref().to_string_lossy().to_string().to_case(case);
    PathBuf::from(converted)
}

impl ToCase for RelativePathBuf {
    type Output = RelativePathBuf;

    fn to_case(self, case: Case) -> Self::Output {
        // Convert each path component to the specified case
        let converted = self
            .components()
            .map(|component| component.as_ref().to_string().to_case(case))
            .collect::<Vec<_>>()
            .join("/");

        // Create a new RelativePathBuf from the converted string
        RelativePathBuf::from(converted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_path_to_snake_case() {
        let path = RelativePathBuf::from("src/HelloWorld/MyComponent");
        let converted = path.to_case(Case::Snake);
        assert_eq!(converted.as_str(), "src/hello_world/my_component");
    }

    #[test]
    fn test_relative_path_to_kebab_case() {
        let path = RelativePathBuf::from("src/HelloWorld/MyComponent");
        let converted = path.to_case(Case::Kebab);
        assert_eq!(converted.as_str(), "src/hello-world/my-component");
    }

    #[test]
    fn test_relative_path_to_pascal_case() {
        let path = RelativePathBuf::from("src/hello_world/my-component");
        let converted = path.to_case(Case::Pascal);
        assert_eq!(converted.as_str(), "src/HelloWorld/MyComponent");
    }

    #[test]
    fn test_relative_path_with_empty_components() {
        let path = RelativePathBuf::from("src//hello_world//my-component");
        let converted = path.to_case(Case::Camel);
        assert_eq!(converted.as_str(), "src/helloWorld/myComponent");
    }

    #[test]
    fn test_convert_path_case() {
        let path = Path::new("src/HelloWorld/MyComponent");
        let converted = convert_path_case(path, Case::Snake);
        assert_eq!(converted.to_str().unwrap(), "src/hello_world/my_component");
    }
}
