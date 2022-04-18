//

use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use walkdir::WalkDir;

fn walk_dir_map<P>(search_dir: P) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    if let Ok(paths) = read_dir(search_dir) {
        let files = paths
            .filter_map(|path| match path {
                Ok(path) => {
                    let path = path.path();
                    if path.is_dir() {
                        Some(walk_dir_map(path))
                    } else {
                        Some(vec![path])
                    }
                }
                _ => None,
            })
            .flatten()
            .collect::<Vec<_>>();
        files
    } else {
        vec![]
    }
}

fn walk_dir_extend<P>(search_dir: P) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    if let Ok(paths) = read_dir(search_dir) {
        let mut files = vec![];
        for path in paths {
            if let Ok(path) = path {
                let path = path.path();
                if path.is_dir() {
                    files.extend(walk_dir_extend(&path));
                } else {
                    files.push(path);
                }
            };
        }
        files
    } else {
        vec![]
    }
}

fn walk_dir_lib<P>(search_dir: P) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    let mut files = vec![];
    for entry in WalkDir::new(search_dir).sort_by_file_name() {
        if let Ok(path) = entry {
            // This does not seem to cause any performance impact
            let path = path.path().to_path_buf();
            if path.is_file() {
                files.push(path);
            }
        };
    }
    files
}

fn criterion_benchmark(c: &mut Criterion) {
    let search_dir = "/Users/akash/Documents/images/";

    let mut group = c.benchmark_group("WalkDir");
    group.bench_with_input(
        BenchmarkId::new("Extend", search_dir),
        search_dir,
        |b, search_dir| b.iter(|| walk_dir_extend(search_dir)),
    );
    group.bench_with_input(
        BenchmarkId::new("Map", search_dir),
        search_dir,
        |b, search_dir| b.iter(|| walk_dir_map(search_dir)),
    );
    group.bench_with_input(
        BenchmarkId::new("Lib", search_dir),
        search_dir,
        |b, search_dir| b.iter(|| walk_dir_lib(search_dir)),
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
