use ansi_term::Color::{Blue, Cyan, Green, White};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 1 { &args[1] } else { "." };

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = fs::symlink_metadata(&path);

                    let file_name = entry.file_name().to_string_lossy().into_owned();

                    if let Ok(meta) = metadata {
                        let file_type = meta.file_type();

                        let colored_name = if file_type.is_symlink() {
                            Cyan.paint(file_name)
                        } else if file_type.is_dir() {
                            Blue.paint(file_name)
                        } else if meta.permissions().mode() & 0o111 != 0 {
                            Green.paint(file_name)
                        } else {
                            White.paint(file_name)
                        };
                        println!("{}", colored_name);
                    } else {
                        println!("{}", White.paint(file_name))
                    }
                }
            }
        }
        Err(e) => eprintln!("Error printing directories: {}", e),
    }
}
