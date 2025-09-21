use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use tempfile::TempDir;

// Import the functions we want to benchmark from the main crate
// We'll need to make these functions public or create a lib.rs

#[derive(Debug, Clone)]
struct FileInfo {
    name: String,
    #[allow(dead_code)]
    path: PathBuf,
    is_dir: bool,
    is_hidden: bool,
    size: u64,
    modified: Option<SystemTime>,
}

impl FileInfo {
    fn from_path(path: PathBuf) -> Result<Self, std::io::Error> {
        let metadata = fs::metadata(&path)?;
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let is_hidden = name.starts_with('.');
        let is_dir = metadata.is_dir();
        let size = metadata.len();
        let modified = metadata.modified().ok();

        Ok(FileInfo {
            name,
            path,
            is_dir,
            is_hidden,
            size,
            modified,
        })
    }
}

fn format_size(size: u64, human: bool) -> String {
    if !human {
        return size.to_string();
    }

    const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:.0}{}", size, UNITS[unit_index])
    } else {
        format!("{:.1}{}", size, UNITS[unit_index])
    }
}

fn format_time(time: Option<SystemTime>) -> String {
    match time {
        Some(t) => {
            let duration = t.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
            let datetime =
                chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0).unwrap_or_default();
            datetime.format("%b %d %H:%M").to_string()
        }
        None => "???".to_string(),
    }
}

fn create_test_directory(num_files: usize) -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create regular files
    for i in 0..num_files / 2 {
        let file_path = temp_dir.path().join(format!("file_{}.txt", i));
        fs::write(&file_path, format!("Content of file {}", i))
            .expect("Failed to create test file");
    }

    // Create hidden files
    for i in 0..num_files / 4 {
        let file_path = temp_dir.path().join(format!(".hidden_{}", i));
        fs::write(&file_path, format!("Hidden content {}", i))
            .expect("Failed to create hidden file");
    }

    // Create directories
    for i in 0..num_files / 4 {
        let dir_path = temp_dir.path().join(format!("dir_{}", i));
        fs::create_dir(&dir_path).expect("Failed to create test directory");
    }

    temp_dir
}

fn bench_format_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("format_size");

    let sizes = vec![
        0,
        512,
        1023,
        1024,
        1536,
        1048576,       // 1 MB
        1073741824,    // 1 GB
        1099511627776, // 1 TB
    ];

    for size in sizes {
        group.bench_with_input(BenchmarkId::new("human_readable", size), &size, |b, &s| {
            b.iter(|| format_size(black_box(s), black_box(true)))
        });

        group.bench_with_input(BenchmarkId::new("raw_bytes", size), &size, |b, &s| {
            b.iter(|| format_size(black_box(s), black_box(false)))
        });
    }

    group.finish();
}

fn bench_format_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("format_time");

    let times = vec![
        None,
        Some(SystemTime::UNIX_EPOCH),
        Some(SystemTime::now()),
        Some(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1640995200)),
    ];

    for (i, time) in times.into_iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("format_time", i), &time, |b, t| {
            b.iter(|| format_time(black_box(*t)))
        });
    }

    group.finish();
}

fn bench_file_info_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_info_creation");

    let temp_dir = create_test_directory(10);
    let paths: Vec<PathBuf> = fs::read_dir(temp_dir.path())
        .expect("Failed to read temp directory")
        .map(|entry| entry.expect("Failed to get directory entry").path())
        .collect();

    group.bench_function("single_file", |b| {
        b.iter(|| {
            if let Some(path) = paths.first() {
                FileInfo::from_path(black_box(path.clone())).ok()
            } else {
                None
            }
        })
    });

    group.bench_function("batch_files", |b| {
        b.iter(|| {
            paths
                .iter()
                .map(|p| FileInfo::from_path(black_box(p.clone())))
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
    });

    group.finish();
}

fn bench_directory_reading(c: &mut Criterion) {
    let mut group = c.benchmark_group("directory_reading");

    let file_counts = vec![10, 50, 100, 500];

    for count in file_counts {
        let temp_dir = create_test_directory(count);

        group.bench_with_input(
            BenchmarkId::new("sequential", count),
            &temp_dir.path(),
            |b, path| {
                b.iter(|| {
                    let entries: Result<Vec<_>, _> = fs::read_dir(black_box(path))
                        .unwrap()
                        .map(|entry| {
                            let entry = entry?;
                            FileInfo::from_path(entry.path())
                        })
                        .collect();
                    entries.ok()
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("parallel", count),
            &temp_dir.path(),
            |b, path| {
                b.iter(|| {
                    use rayon::prelude::*;
                    let entries: Result<Vec<_>, _> = fs::read_dir(black_box(path))
                        .unwrap()
                        .par_bridge()
                        .map(|entry| {
                            let entry = entry?;
                            FileInfo::from_path(entry.path())
                        })
                        .collect();
                    entries.ok()
                })
            },
        );
    }

    group.finish();
}

fn bench_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");

    let temp_dir = create_test_directory(100);
    let mut entries: Vec<FileInfo> = fs::read_dir(temp_dir.path())
        .expect("Failed to read temp directory")
        .map(|entry| {
            let entry = entry.expect("Failed to get directory entry");
            FileInfo::from_path(entry.path()).expect("Failed to create FileInfo")
        })
        .collect();

    group.bench_function("sort_by_name", |b| {
        b.iter(|| {
            let mut entries_copy = black_box(entries.clone());
            entries_copy.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            entries_copy
        })
    });

    group.bench_function("sort_by_time", |b| {
        b.iter(|| {
            let mut entries_copy = black_box(entries.clone());
            entries_copy.sort_by(|a, b| {
                b.modified
                    .unwrap_or(SystemTime::UNIX_EPOCH)
                    .cmp(&a.modified.unwrap_or(SystemTime::UNIX_EPOCH))
            });
            entries_copy
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_format_size,
    bench_format_time,
    bench_file_info_creation,
    bench_directory_reading,
    bench_sorting
);
criterion_main!(benches);
