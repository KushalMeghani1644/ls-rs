# ls-rs

A blazing-fast and colorful `ls` alternative written in Rust 🦀

Easily list directory contents with intuitive color coding, smart formatting, and robust error handling.

## ✨ Features

- **Color-coded output** based on file type:
  - 🔵 **Blue**: Directories  
  - 🔗 **Cyan**: Symlinks
  - 🟢 **Green**: Executable files
  - 🟡 **Yellow**: Image files (jpg, png, gif, etc.)
  - ⚪ **White**: Regular files
- **Smart column layout** that adapts to your terminal width
- **Hidden file support** with `-a/--all` flag
- **Robust error handling** with helpful error messages
- **Fast performance** thanks to Rust's zero-cost abstractions
- **Cross-platform compatibility**
- **Intuitive sorting**: non-hidden files first, case-insensitive alphabetical order

## 🚀 Installation

### From [crates.io](https://crates.io/crates/ls-rs)
```bash
cargo install ls-rs
```

### From source
```bash
git clone https://github.com/KushalMeghani1644/ls-rs
cd ls-rs
cargo build --release
```

## 📖 Usage

```bash
# List current directory
ls-rs

# List specific directory
ls-rs /path/to/directory

# Show hidden files
ls-rs -a
ls-rs --all

# Show help
ls-rs -h
ls-rs --help
```

## 🎨 Color Scheme

| File Type | Color | Example |
|-----------|-------|---------|
| Directory | Blue | `Documents/` |
| Symlink | Cyan | `link -> target` |
| Executable | Green | `script.sh` |
| Image | Yellow | `photo.jpg` |
| Regular file | White | `readme.txt` |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the GPLv3 License - see the [LICENSE](LICENSE) file for details.
