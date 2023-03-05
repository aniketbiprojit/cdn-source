use std::{fs, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct File {
    name: String,
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
                folder.files.push(File { name: file_path });
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
