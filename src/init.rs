use std::{env, fs, path::PathBuf, time::SystemTime};

use crate::hashing::{self, save_csv, CsvData};

pub fn init(path: Option<PathBuf>) {
    let path: PathBuf = match path {
        Some(p) => p,
        None => env::current_dir().unwrap(),
    };
    check_init_folder_exist(&path);
}

fn check_init_folder_exist(path: &PathBuf) {
    let mut init_path: PathBuf = path.clone();
    init_path.push(".file-diff");
    if init_path.exists() {
        println!("record folder existed");
    } else {
        fs::create_dir(&init_path).unwrap();
        println!("folder created");


        let now = SystemTime::now();
        println!("Current date and time: {:?}", now);
        match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => init_path.push(format!("{}_init", n.as_secs())),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
        
        let mut data: Vec<CsvData> = Vec::new();
        save_csv(init_path, data);
    }
}
