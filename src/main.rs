use ansi_term::Color::{Blue, Cyan, Green, White, Yellow};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use terminal_size::{Width, terminal_size};

#[derive(Debug)]
struct FileEntry {
    name: String,
    styled_name: String,
    is_hidden: bool,
}

fn get_terminal_width() -> usize {
    terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80)
}

fn style_filename(file_name: &str, path: &Path) -> String {
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
                // Check for common file extensions
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

fn parse_args() -> (String, bool) {
    let args: Vec<String> = env::args().collect();
    let mut dir = ".".to_string();
    let mut show_hidden = false;

    for arg in args.iter().skip(1) {
        if arg == "-a" || arg == "--all" {
            show_hidden = true;
        } else if arg == "-h" || arg == "--help" {
            println!("Usage: ls-rs [OPTIONS] [DIRECTORY]");
            println!("Options:");
            println!("  -a, --all    Show hidden files");
            println!("  -h, --help   Show this help message");
            std::process::exit(0);
        } else if !arg.starts_with('-') {
            dir = arg.clone();
        }
    }

    (dir, show_hidden)
}

fn main() {
    let (dir, show_hidden) = parse_args();

    // Validate directory path
    if !Path::new(&dir).exists() {
        eprintln!("ls-rs: cannot access '{}': No such file or directory", dir);
        std::process::exit(1);
    }

    if !Path::new(&dir).is_dir() {
        eprintln!("ls-rs: '{}': Not a directory", dir);
        std::process::exit(1);
    }

    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries
            .filter_map(|entry_result| match entry_result {
                Ok(entry) => {
                    let file_name = entry.file_name().to_string_lossy().into_owned();
                    if should_show_file(&file_name, show_hidden) {
                        let styled_name = style_filename(&file_name, &entry.path());
                        Some(FileEntry {
                            name: file_name.clone(),
                            styled_name,
                            is_hidden: file_name.starts_with('.'),
                        })
                    } else {
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Error reading entry: {}", e);
                    None
                }
            })
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("ls-rs: cannot read directory '{}': {}", dir, e);
            std::process::exit(1);
        }
    };

    if entries.is_empty() {
        return;
    }

    // Sort: directories first, then by name (case-insensitive)
    let mut sorted_entries = entries;
    sorted_entries.sort_by(|a, b| {
        // First compare by whether they're hidden (non-hidden first)
        match (a.is_hidden, b.is_hidden) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => {
                // Then sort alphabetically (case-insensitive)
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        }
    });

    let plain_names: Vec<&str> = sorted_entries.iter().map(|e| e.name.as_str()).collect();
    let styled_names: Vec<&str> = sorted_entries
        .iter()
        .map(|e| e.styled_name.as_str())
        .collect();

    let max_len = plain_names.iter().map(|name| name.len()).max().unwrap_or(0);
    let col_width = max_len + 2;
    let term_width = get_terminal_width();
    let cols = if col_width == 0 {
        1
    } else {
        (term_width / col_width).max(1)
    };

    if cols == 1 || styled_names.len() <= cols {
        // Single column or few enough items to fit in one row
        for styled_name in &styled_names {
            println!("{}", styled_name);
        }
    } else {
        let rows = (styled_names.len() + cols - 1) / cols;
        for row in 0..rows {
            let mut line_items = Vec::new();

            for col in 0..cols {
                let idx = col * rows + row;
                if idx < styled_names.len() {
                    line_items.push((styled_names[idx], plain_names[idx].len()));
                }
            }

            // Print the line
            for (i, (styled_name, plain_len)) in line_items.iter().enumerate() {
                print!("{}", styled_name);

                // Add padding if not the last item in the line
                if i < line_items.len() - 1 {
                    let padding = col_width - plain_len;
                    print!("{}", " ".repeat(padding));
                }
            }
            println!();
        }
    }
}
