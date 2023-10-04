use std::{env, path::PathBuf};
use time::OffsetDateTime;

use crate::hashing::{self, CsvData, save_csv};

pub fn generate(path: Option<PathBuf>, item: String, export_path: Option<PathBuf>) {
    let path: PathBuf = match path {
        Some(p) => p,
        None => env::current_dir().unwrap(),
    };

    let mut init_path = path.clone();
    init_path.push(".file-diff");

    if init_path.exists() {
        let mut item_path = init_path.clone();
        item_path.push(&item);

        if !item_path.exists() {
            println!("{} not exist", item);
        }

        let result:Vec<CsvData> = CsvData::get_data_from_csv(&item_path.to_str().unwrap());
        println!("| path | hash | last modified|");
        println!("| ----- | ----- | ----- |");

        for item in &result {
            println!("| {} | {} | {} |", item.path, item.hash, item.last_modified);

        }

        match export_path{
            Some(p) => {
                let mut csv_path = p.clone();

                
                csv_path.push(format!("{}.csv", item));


                save_csv(csv_path, result)
            },
            None => {},
        }

    } else {
        println!("record folder not exist");
    }
}
