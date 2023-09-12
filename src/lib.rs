#![allow(unused_variables)]

use std::{fs, path::Path};

/// Recursively deletes all files and directories within a directory.
///
/// # Arguments
///
/// * `dir_path` - A `Path` reference to the directory to delete the contents of.
///
pub fn delete_dir_contents(dir_path: &Path) {
    if dir_path.exists() && dir_path.is_dir() {
        let inner_paths = fs::read_dir(dir_path).unwrap();
        for inner_path in inner_paths {
            let inner_path = inner_path.unwrap();
            if inner_path.file_type().unwrap().is_dir() {
                let inner_dir_name = inner_path.file_name();
                let inner_dir_path = dir_path.join(inner_dir_name);
                delete_dir_contents(&inner_dir_path);
            } else {
                let file_name = inner_path.file_name();
                let file_path = dir_path.join(file_name);
                if file_path.exists() && file_path.is_file() {
                    fs::remove_file(file_path).unwrap();
                }
            }
        }
    }
}

/// Checks if a file or directory exists at the given path.
///
/// # Arguments
///
/// * `path` - A `Path` reference to the file or directory to check for existence.
///
/// # Returns
///
/// A `Result` containing a `bool` indicating whether the file or directory exists (`true`) or not (`false`), or an `std::io::Error` if an I/O error occurred.
///
pub fn check_if_path_exists(path: &Path) -> Result<bool, std::io::Error> {
    match fs::metadata(path) {
        Ok(metadata) => Ok(true),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok(false),
            _ => Err(e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_check_if_path_exists() {
        let path = PathBuf::from("src/lib.rs");
        let result = check_if_path_exists(&path);
        assert!(
            result.is_ok(),
            "Expected Ok result, but got an error: {:?}",
            result
        );
        assert_eq!(result.unwrap(), true, "Expected true, but got false");

        let path = PathBuf::from("src/lib1.rs");
        let result = check_if_path_exists(&path);
        assert!(
            result.is_ok(),
            "Expected Ok result, but got an error: {:?}",
            result
        );
        assert_eq!(result.unwrap(), false, "Expected false, but got true");
    }
}
