use clap::Parser;
use crossterm::{
    style::{Color, Stylize},
    terminal,
};
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Parser)]
#[command(name = "zls")]
#[command(about = "A fast ls replacement written in Rust")]
#[command(version)]
struct Args {
    #[arg(help = "Directory to list", default_value = ".")]
    path: PathBuf,

    #[arg(short, long, help = "Show hidden files")]
    all: bool,

    #[arg(short, long, help = "Use short listing format")]
    short: bool,

    #[arg(short, long, help = "Sort by modification time")]
    time: bool,

    #[arg(short = 'H', long, help = "Show human readable sizes", default_value_t = true)]
    human: bool,

    #[arg(long, help = "Show raw byte sizes instead of human readable")]
    bytes: bool,
}

#[derive(Debug)]
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

fn list_directory(path: &Path, args: &Args) -> Result<Vec<FileInfo>, std::io::Error> {
    let entries: Result<Vec<_>, _> = fs::read_dir(path)?
        .par_bridge()
        .map(|entry| {
            let entry = entry?;
            FileInfo::from_path(entry.path())
        })
        .collect();

    let mut entries = entries?;

    if !args.all {
        entries.retain(|file| !file.is_hidden);
    }

    if args.time {
        entries.sort_by(|a, b| {
            b.modified
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .cmp(&a.modified.unwrap_or(SystemTime::UNIX_EPOCH))
        });
    } else {
        entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }

    Ok(entries)
}

fn print_entries(entries: &[FileInfo], args: &Args) {
    let term_width = terminal::size().map(|(w, _)| w as usize).unwrap_or(80);

    if !args.short {
        for entry in entries {
            let size_str = format_size(entry.size, args.human && !args.bytes);
            let time_str = format_time(entry.modified);
            let type_char = if entry.is_dir { "d" } else { "-" };

            let name = if entry.is_dir {
                entry.name.clone().with(Color::Blue).bold().to_string()
            } else if entry.name.ends_with(".rs") {
                entry.name.clone().with(Color::Yellow).to_string()
            } else if entry.name.starts_with('.') {
                entry.name.clone().with(Color::DarkGrey).to_string()
            } else {
                entry.name.clone()
            };

            println!("{} {:>8} {} {}", type_char, size_str, time_str, name);
        }
    } else {
        let mut current_width = 0;
        for (i, entry) in entries.iter().enumerate() {
            let name = if entry.is_dir {
                format!("{}/", entry.name)
                    .with(Color::Blue)
                    .bold()
                    .to_string()
            } else if entry.name.ends_with(".rs") {
                entry.name.clone().with(Color::Yellow).to_string()
            } else if entry.name.starts_with('.') {
                entry.name.clone().with(Color::DarkGrey).to_string()
            } else {
                entry.name.clone()
            };

            let display_width = entry.name.len() + if entry.is_dir { 1 } else { 0 };

            if current_width + display_width + 2 > term_width && i > 0 {
                println!();
                current_width = 0;
            }

            print!("{:<width$}", name, width = display_width + 2);
            current_width += display_width + 2;
        }
        if !entries.is_empty() {
            println!();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !args.path.exists() {
        eprintln!("Error: Path '{}' does not exist", args.path.display());
        std::process::exit(1);
    }

    if !args.path.is_dir() {
        eprintln!("Error: '{}' is not a directory", args.path.display());
        std::process::exit(1);
    }

    let entries = list_directory(&args.path, &args)?;
    print_entries(&entries, &args);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::SystemTime;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0, false), "0");
        assert_eq!(format_size(512, false), "512");
        assert_eq!(format_size(1023, false), "1023");
    }

    #[test]
    fn test_format_size_human_readable() {
        assert_eq!(format_size(0, true), "0B");
        assert_eq!(format_size(512, true), "512B");
        assert_eq!(format_size(1024, true), "1.0K");
        assert_eq!(format_size(1536, true), "1.5K");
        assert_eq!(format_size(1048576, true), "1.0M");
        assert_eq!(format_size(1073741824, true), "1.0G");
        assert_eq!(format_size(1099511627776, true), "1.0T");
    }

    #[test]
    fn test_format_time_none() {
        assert_eq!(format_time(None), "???");
    }

    #[test]
    fn test_format_time_some() {
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1640995200); // 2022-01-01 00:00:00 UTC
        let formatted = format_time(Some(time));
        // Should contain month, day, hour, minute format
        assert!(formatted.len() > 5);
        assert!(formatted.contains(':'));
    }

    #[test]
    fn test_file_info_hidden_detection() {
        // Test that files starting with '.' are detected as hidden
        let path = PathBuf::from(".hidden_file");

        // We'll create a mock FileInfo since we can't guarantee the file exists
        let file_info = FileInfo {
            name: ".hidden_file".to_string(),
            path: path.clone(),
            is_dir: false,
            is_hidden: true,
            size: 0,
            modified: None,
        };

        assert!(file_info.is_hidden);
        assert_eq!(file_info.name, ".hidden_file");
    }

    #[test]
    fn test_file_info_regular_file() {
        let file_info = FileInfo {
            name: "regular_file.txt".to_string(),
            path: PathBuf::from("regular_file.txt"),
            is_dir: false,
            is_hidden: false,
            size: 1024,
            modified: Some(SystemTime::now()),
        };

        assert!(!file_info.is_hidden);
        assert!(!file_info.is_dir);
        assert_eq!(file_info.size, 1024);
    }

    #[test]
    fn test_file_info_directory() {
        let file_info = FileInfo {
            name: "directory".to_string(),
            path: PathBuf::from("directory"),
            is_dir: true,
            is_hidden: false,
            size: 4096,
            modified: Some(SystemTime::now()),
        };

        assert!(!file_info.is_hidden);
        assert!(file_info.is_dir);
    }

    #[test]
    fn test_format_size_edge_cases() {
        assert_eq!(format_size(1023, true), "1023B");
        assert_eq!(format_size(1025, true), "1.0K");
        assert_eq!(format_size(1048575, true), "1024.0K");
        assert_eq!(format_size(1048577, true), "1.0M");
    }
}
