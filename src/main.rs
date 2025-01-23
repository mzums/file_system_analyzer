use clap::{Arg, Command};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io;
use std::path::Path;
use serde_yaml;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileInfo {
    name: String,
    size: u64,
    folder: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DirectorySummary {
    directory: String,
    total_size: u64,
    file_count: usize,
    files: Vec<FileInfo>,
    subdirectories: Vec<DirectorySummary>,
}

fn export_to_yaml(data: &[DirectorySummary], output_path: Option<&str>) {
    let writer: Box<dyn io::Write> = match output_path {
        Some(path) => {
            let file = fs::File::create(path).expect("Failed to create file");
            Box::new(file)
        }
        None => Box::new(io::stdout()),
    };

    serde_yaml::to_writer(writer, &data).expect("Failed writing data to YAML file");
}

fn should_skip_folder(folder_name: &str, skip_folders: &[String]) -> bool {
    skip_folders.iter().any(|skip| skip == folder_name)
}

fn print_folder_tree(indent: usize, path: &Path) {
    let indent_str = "  ".repeat(indent);
    println!("{}{}", indent_str, path.display());
}

fn analyze_directory(
    path: &Path,
    skip_folders: &[String],
    parent_folder: &str,
    indent: usize,
    base_path: &Path,
) -> DirectorySummary {
    let mut total_size = 0;
    let mut file_count = 0;
    let mut files = Vec::new();
    let mut subdirectories = Vec::new();

    if path.is_dir() {
        let folder_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        if should_skip_folder(&folder_name, skip_folders) {
            return DirectorySummary {
                directory: path.to_string_lossy().to_string(),
                total_size: 0,
                file_count: 0,
                files: vec![],
                subdirectories: vec![],
            };
        }

        print_folder_tree(indent, path);

        for entry in fs::read_dir(path).expect("Failed to read folder") {
            let entry = entry.expect("Error reading directory entry");
            let metadata = entry.metadata().expect("Failed to retrieve metadata");
            let file_name = entry.file_name().into_string().unwrap_or_default();
            let entry_path = entry.path();

            if metadata.is_file() {
                let file_size = metadata.len();
                total_size += file_size;
                file_count += 1;

                let relative_folder = entry_path.strip_prefix(base_path).unwrap_or(&entry_path);
                files.push(FileInfo {
                    name: file_name,
                    size: file_size,
                    folder: relative_folder.to_string_lossy().to_string(),
                });
            } else if metadata.is_dir() {
                let subdir_summary = analyze_directory(
                    &entry_path,
                    skip_folders,
                    &format!("{}\\{}", parent_folder, file_name),
                    indent + 1,
                    base_path,
                );
                total_size += subdir_summary.total_size;
                file_count += subdir_summary.file_count;
                files.extend(subdir_summary.files.clone());
                subdirectories.push(subdir_summary.clone());
            }
        }
    }

    DirectorySummary {
        directory: path.to_string_lossy().to_string(),
        total_size,
        file_count,
        files,
        subdirectories,
    }
}

fn main() {
    let matches = Command::new("FileSystemAnalyzer")
        .version("1.0")
        .author("mzums")
        .about("Analyzes file structure and writes results to YAML")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("KATALOG")
                .help("The directory path to analyze")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("PLIK")
                .help("The path of the YAML file to save the results")
                .takes_value(true),
        )
        .arg(
            Arg::new("skip")
                .long("skip")
                .value_name("FOLDER")
                .help("List of folders to skip (e.g. target, .git)")
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap();
    let output_path = matches.value_of("output");
    let skip_folders: Vec<String> = matches
        .values_of("skip")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

    let base_path = Path::new(directory);

    let summary = analyze_directory(base_path, &skip_folders, directory, 0, base_path);

    export_to_yaml(&[summary], output_path);

    println!("Analysis complete!");
    if let Some(path) = output_path {
        println!("Data saved in file: {}", path);
    } else {
        println!("Data displayed in the terminal.");
    }
}
