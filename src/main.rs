use ansi_term::Color::{Blue, Cyan, Green, White};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use terminal_size::{Width, terminal_size};

fn get_terminal_width() -> usize {
    terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 1 { &args[1] } else { "." };

    let mut entries = match fs::read_dir(dir) {
        Ok(entries) => entries.filter_map(|e| e.ok()).collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    // Sort alphabetically
    entries.sort_by_key(|e| e.file_name());

    let plain_names: Vec<String> = entries
        .iter()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    let styled_name: Vec<String> = entries
        .iter()
        .map(|entry| {
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path);
            let file_name = entry.file_name().to_string_lossy().into_owned();

            if let Ok(meta) = metadata {
                let file_type = meta.file_type();
                if file_type.is_symlink() {
                    Cyan.paint(file_name).to_string()
                } else if file_type.is_dir() {
                    Blue.paint(file_name).to_string()
                } else if meta.permissions().mode() & 0o111 != 0 {
                    Green.paint(file_name).to_string()
                } else {
                    White.paint(file_name).to_string()
                }
            } else {
                White.paint(file_name).to_string()
            }
        })
        .collect();

    let max_len = plain_names.iter().map(|name| name.len()).max().unwrap_or(0);
    let col_width = max_len + 2;
    let term_width = get_terminal_width();
    let cols = if col_width == 0 {
        1
    } else {
        (term_width / col_width).max(1)
    };
    if cols == 1 {
        for styled_name in &styled_name {
            println!("{}", styled_name);
        }
    } else {
        let rows = (styled_name.len() + cols - 1) / cols;

        for row in 0..rows {
            for col in 0..cols {
                let idx = col * rows + row;
                if idx < styled_name.len() {
                    // Print the styled name
                    print!("{}", styled_name[idx]);

                    // Add padding (but not after the last column with content)
                    if col < cols - 1 {
                        // Check if there's content in the next columns for this row
                        let mut has_next = false;
                        for next_col in (col + 1)..cols {
                            let next_idx = next_col * rows + row;
                            if next_idx < styled_name.len() {
                                has_next = true;
                                break;
                            }
                        }

                        if has_next {
                            let padding = col_width - plain_names[idx].len();
                            print!("{}", " ".repeat(padding));
                        }
                    }
                }
            }
            println!();
        }
    }
}
