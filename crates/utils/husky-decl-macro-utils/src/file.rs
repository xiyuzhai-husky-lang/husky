pub use husky_path_utils::rel::get_path_relative_to_current;

#[macro_export]
macro_rules! file_rel_curr {
    () => {
        $crate::file::get_path_relative_to_current(std::path::Path::from(file!()))
    };
}
