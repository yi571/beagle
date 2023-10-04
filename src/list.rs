use std::{env, path::PathBuf};

use walkdir::WalkDir;

pub fn get_csv_list(path: Option<PathBuf>) {

    let path: PathBuf = match path {
        Some(p) => p,
        None => env::current_dir().unwrap(),
    };

    let mut init_path = path.clone();
    init_path.push(".file-diff");

    if init_path.exists() {
        let walker = WalkDir::new(&init_path).into_iter();
        for file in walker.filter_map(|e| e.ok()) {
            if file.metadata().unwrap().is_file() && file.file_name().to_string_lossy() != String::from("record.redb") {
                let file_path: String = String::from(file.file_name().to_string_lossy());

                let mut file_path = PathBuf::from(file_path);
                file_path.set_extension("");
                println!("{}", file_path.to_string_lossy());
            }
        }
    } else {
        println!("folder not exist");
    }
}
