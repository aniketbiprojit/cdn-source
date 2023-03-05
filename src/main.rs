use std::{fs, io::Write, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct File {
    name: String,
    versions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<Folder>,
}

fn recursively_get_folder_paths(path: String) -> Result<Folder, ()> {
    let files = fs::read_dir(&path).unwrap();
    let mut folder = Folder {
        name: path.clone(),
        files: Vec::new(),
        folders: Vec::new(),
    };
    for file in files {
        let filename = file.unwrap().file_name();
        let filename = filename.to_str();
        if let Some(filename) = filename {
            let file_path = format!("{}/{}", path, filename);
            let metadata = fs::metadata(&file_path).unwrap();
            if metadata.is_dir() {
                folder
                    .folders
                    .push(recursively_get_folder_paths(file_path).unwrap());
            } else {
                let log_data = Command::new("git")
                    .args(["log", "--oneline", "--all", &file_path])
                    .output()
                    .expect("Failed to execute git log command");

                let from_utf8_lossy = String::from_utf8_lossy(&log_data.stdout);
                let splits = from_utf8_lossy.split_terminator("\n");

                let mut versions = Vec::new();
                for split in splits {
                    if split.len() > 0 {
                        let split = split.split(" ").collect::<Vec<&str>>();
                        versions.push(split[0].to_string());
                    }
                }

                folder.files.push(File {
                    name: file_path,
                    versions: versions,
                });
            }
        }
    }
    Ok(folder)
}

fn main() {
    let base_dir = "./files";

    let folder = recursively_get_folder_paths(base_dir.to_string()).unwrap();

    let mut fp = fs::File::create("manifest.json").unwrap();

    let data = serde_json::to_string(&folder).expect("Failed to serialize");
    fp.write_all(data.as_bytes()).unwrap();
}
