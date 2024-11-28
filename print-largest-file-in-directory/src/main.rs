use std::{env, fs};

fn get_current_working_directory() -> String {
    let dir_path_buf = env::current_dir().expect("Error getting current working directory");
    dir_path_buf.to_str().expect("Error converting path to string").to_string()
}

fn main() {
    let dir = get_current_working_directory();
    let file_entries = fs::read_dir(dir).expect("Invalid directory");
    let mut largest_file: Option<String> = None;
    let mut largest_size: u64 = 0;

    for entry in file_entries {
        let entry = entry.expect("Error reading entry");
        let path = entry.path();
        let metadata = fs::metadata(&path).expect("Error reading metadata");
        let size: u64 = metadata.len();
        if size > largest_size {
            largest_size = size;
            largest_file = Some(path.to_str().expect("Error converting path to string").to_string());
        }
    }

    if let Some(file) = largest_file {
        println!("Largest file: {:?}", file);
    } else {
        println!("No files found");
    }
}
