use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub fn walk_dir<P: AsRef<Path>>(search_dir: P, sort: bool) -> Vec<PathBuf> {
    let paths = read_dir(search_dir);
    if paths.is_err() {
        return vec![];
    };
    let paths = paths.unwrap();
    let mut files = vec![];
    for path in paths {
        if let Ok(path) = path {
            let path = path.path();
            if path.is_dir() {
                files.extend(walk_dir(&path, false));
            } else {
                files.push(path);
            }
        };
    }
    if sort {
        files.sort()
    }
    files
}
