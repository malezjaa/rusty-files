# `rusty-files`

Rusty files is a simple collection of file functions that I use in my projects.

## Instalation

```bash
cargo install rusty-files
```

## Functons

##### `delete_dir_contents` - Recursively deletes all files and directories within a directory.

##### Arguments

- `dir_path` - A `Path` reference to the directory to delete the contents of.

```rust
use rusty_files::delete_dir_contents;
use std::path::Path;

fn main() {
    let dir_path = Path::new("path/to/dir");
    delete_dir_contents(&dir_path);
}
```

##### `check_if_path_exists` - Checks if a file or directory exists at the given path.

##### Arguments

- `path` - A `Path` reference to the file or directory to check for existence.

##### Returns

A `Result` containing a `bool` indicating whether the file or directory exists (`true`) or not (`false`), or an `std::io::Error` if an I/O error occurred.

```rust
use rusty_files::check_if_path_exists;
use std::path::Path;

fn main() {
   let path = Path::new("path/to/file");
   let path_exists = check_if_path_exists(&path).unwrap();
   println!("{}", path_exists);
```

## Authors

- [@malezjaa](https://www.github.com/malezjaa)

## Badges

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

![GitHub Sponsors](https://img.shields.io/github/sponsors/malezjaa)
