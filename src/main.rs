use ansi_term::Color::{Blue, Cyan, Green, White, Yellow};
use atty::Stream;
use clap::Parser;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use terminal_size::{Width, terminal_size};

#[derive(Debug)]
struct FileEntry {
    name: String,
    styled_name: String,
    is_hidden: bool,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show hidden files
    #[arg(short, long)]
    all: bool,

    /// Directory to list
    #[arg(default_value = ".")]
    directory: String,
}

fn get_terminal_width() -> usize {
    terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80)
}

fn style_filename(file_name: &str, path: &Path, color_output: bool) -> String {
    if !color_output {
        return file_name.to_string();
    }

    match fs::symlink_metadata(path) {
        Ok(meta) => {
            let file_type = meta.file_type();
            if file_type.is_symlink() {
                Cyan.paint(file_name).to_string()
            } else if file_type.is_dir() {
                Blue.paint(file_name).to_string()
            } else if meta.permissions().mode() & 0o111 != 0 {
                Green.paint(file_name).to_string()
            } else {
                match path.extension().and_then(|ext| ext.to_str()) {
                    Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("bmp")
                    | Some("svg") => Yellow.paint(file_name).to_string(),
                    _ => White.paint(file_name).to_string(),
                }
            }
        }
        Err(_) => White.paint(file_name).to_string(),
    }
}

fn should_show_file(name: &str, show_hidden: bool) -> bool {
    show_hidden || !name.starts_with('.')
}

fn print_columns(entries: &[FileEntry], term_width: usize) {
    if entries.is_empty() {
        return;
    }

    let plain_names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    let styled_names: Vec<&str> = entries.iter().map(|e| e.styled_name.as_str()).collect();

    let max_len = plain_names.iter().map(|name| name.len()).max().unwrap_or(0);
    let col_width = max_len + 2;
    let cols = if col_width == 0 {
        1
    } else {
        (term_width / col_width).max(1)
    };

    if cols == 1 || styled_names.len() <= cols {
        for styled_name in &styled_names {
            println!("{}", styled_name);
        }
    } else {
        let rows = (styled_names.len() + cols - 1) / cols;
        for row in 0..rows {
            for col in 0..cols {
                let idx = col * rows + row;
                if idx < styled_names.len() {
                    let plain_len = plain_names[idx].len();
                    print!("{}", styled_names[idx]);
                    if col < cols - 1 && idx + rows < styled_names.len() {
                        let padding = col_width - plain_len;
                        print!("{}", " ".repeat(padding));
                    }
                }
            }
            println!();
        }
    }
}

fn main() {
    let args = Args::parse();
    let dir_path = PathBuf::from(&args.directory);

    if !dir_path.exists() {
        eprintln!(
            "ls-rs: cannot access '{}': No such file or directory",
            dir_path.display()
        );
        std::process::exit(1);
    }

    if !dir_path.is_dir() {
        eprintln!("ls-rs: '{}': Not a directory", dir_path.display());
        std::process::exit(1);
    }

    let color_output = atty::is(Stream::Stdout);

    let mut entries: Vec<FileEntry> = fs::read_dir(&dir_path)
        .unwrap_or_else(|e| {
            eprintln!(
                "ls-rs: cannot read directory '{}': {}",
                dir_path.display(),
                e
            );
            std::process::exit(1);
        })
        .filter_map(|entry_res| match entry_res {
            Ok(entry) => {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                if should_show_file(&file_name, args.all) {
                    Some(FileEntry {
                        is_hidden: file_name.starts_with('.'),
                        styled_name: style_filename(&file_name, &entry.path(), color_output),
                        name: file_name,
                    })
                } else {
                    None
                }
            }
            Err(e) => {
                eprintln!("Warning: error reading entry: {}", e);
                None
            }
        })
        .collect();

    // Sort: hidden files last, then alphabetically (case-insensitive)
    entries.sort_by(|a, b| {
        a.is_hidden
            .cmp(&b.is_hidden)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    let term_width = get_terminal_width();
    print_columns(&entries, term_width);
}
