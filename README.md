# File System Analyzer 🗂️

A simple command-line tool written in Rust to analyze the contents of a directory, including subdirectories, and output the results into a CSV file. The program calculates the total size of files, counts files and folders, and lists all files with their sizes. 📊

## Features ✨

- Recursively analyzes directories and subdirectories 🏠➡️📁
- Counts the number of files and folders 📂
- Calculates the total size of files 🧮
- Exports the results to CSV for easy analysis or sharing 📥
- Displays the output to stdout or writes to a specified CSV file 📑

---

## Requirements ⚙️

- Rust programming language (Rust 1.XX or later)
- `cargo` for building and running the project

---

## Installation 🛠️

To build and run the project, follow these steps:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/mzums/file_system_analyzer
   cd file_system_analyzer
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

---

## Usage 📈

### Analyze a directory and output the results to the terminal:
```bash
cargo run -- --directory /path/to/your/directory
```

### Analyze a directory skipping picked subfolders:
```bash
cargo run -- --skip directory_name
```

### Analyze a directory and save the results to a CSV file:
```bash
cargo run -- --directory /path/to/your/directory --output output.yaml
```

### CLI Options 📝

- `--directory (-d)` : The path to the directory you want to analyze (required).
- `--output (-o)` : The output path for the CSV file. If not provided, the results will be displayed in the terminal.

---

## Example Usage 🎯

1. **Running the program to analyze a directory**:
   ```bash
   cargo run -- --directory ./src
   ```

   Output (in the terminal or CSV):
   ```csv
   name,size
   main.rs,1024
   lib.rs,2048
   subfolder/file1.txt,512
   subfolder/file2.txt,1024
   ```

2. **Saving the results to a CSV file**:
   ```bash
   cargo run -- --directory ./src --output results.yaml
   ```

---

## License ⚖️

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Contributing 🤝

Feel free to contribute by opening issues, creating pull requests, or suggesting new features! 💡

---

### Enjoy using FileSystemAnalyzer! 🚀