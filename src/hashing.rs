use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs::File, io, path::PathBuf};

#[derive(Serialize, Debug, Deserialize)]
pub struct CsvData {
    pub path: String,
    pub hash: String,
    pub last_modified: String,
}

pub struct CompareCsvData {
    pub path: String,
    pub origin_hash: String,
    pub target_hash: String,
}

pub fn get_file_hash(file_path: String, modified_time: String) -> CsvData {
    let mut file = File::open(&file_path).unwrap();
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).unwrap();
    let result = hasher.finalize();
    let hash_data = format!("{:x}", result);

    CsvData {
        path: file_path,
        hash: hash_data,
        last_modified: modified_time,
    }
}

pub fn save_csv(csv_path: PathBuf, data: Vec<CsvData>) {
    let mut wtr = csv::Writer::from_path(csv_path).expect("csv init error");
    wtr.write_record(&["path", "hash", "last modified"])
        .expect("write head error");

    for i in data {
        wtr.write_record(&[i.path, i.hash])
            .expect("write data error");
    }

    wtr.flush().expect("save csv error")
}

impl CsvData {
    pub fn get_data_from_csv(csv_path: &str) -> Vec<CsvData> {
        // Open file
        let file = std::fs::File::open(csv_path).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut data: Vec<CsvData> = Vec::new();

        // push all the records
        for result in rdr.records().into_iter() {
            let record = result.unwrap();

            let csv_data = Self::get_stuct(&record);
            data.push(csv_data);
        }

        return data;
    }

    fn get_stuct(row: &csv::StringRecord) -> CsvData {
        let csv_data = CsvData {
            path: row[0].to_string(),
            hash: row[1].to_string(),
            last_modified: row[2].to_string(),
        };
        csv_data
    }
}

impl CompareCsvData {
    pub fn new(origin_csv_path: &str, target_csv_path: &str) -> Vec<CompareCsvData> {
        let origin_csv_data = Self::get_hashmap_from_csv(origin_csv_path);
        let target_csv_data = Self::get_hashmap_from_csv(target_csv_path);

        let mut result: Vec<CompareCsvData> = Vec::new();

        // let mut removed = HashMap::new();

        for (key, value) in &origin_csv_data {
            if !target_csv_data.contains_key(key) {
                // removed.insert(key, value);
                let data = CompareCsvData {
                    path: key.to_string(),
                    origin_hash: value.to_string(),
                    target_hash: "NULL".to_string(),
                };
                result.push(data);
            }
        }

        // let mut updated = HashMap::new();
        for (key, new_value) in &target_csv_data {
            if let Some(old_value) = origin_csv_data.get(key) {
                if new_value != old_value {
                    // updated.insert(key, new_value);
                    let data = CompareCsvData {
                        path: key.to_string(),
                        origin_hash: old_value.to_string(),
                        target_hash: new_value.to_string(),
                    };
                    result.push(data);
                }
            } else {
                // updated.insert(key, new_value);
                let data = CompareCsvData {
                    path: key.to_string(),
                    origin_hash: "NULL".to_string(),
                    target_hash: new_value.to_string(),
                };
                result.push(data);
            }
        }

        result
    }

    pub fn save_to_csv(
        csv_path: PathBuf,
        data: Vec<CompareCsvData>,
        origin_item: String,
        target_item: String,
    ) {
        let mut wtr = csv::Writer::from_path(csv_path).expect("csv init error");
        wtr.write_record(&["path", &origin_item, &target_item])
            .expect("write head error");

        for i in data {
            wtr.write_record(&[i.path, i.origin_hash, i.target_hash])
                .expect("write data error");
        }

        wtr.flush().expect("save csv error")
    }

    fn get_hashmap_from_csv(csv_path: &str) -> HashMap<String, String> {
        // Open file
        let file = std::fs::File::open(csv_path).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut csv_data = HashMap::new();

        for result in rdr.records().into_iter() {
            let record = result.unwrap();

            csv_data.insert(record[0].to_string(), record[1].to_string());
        }

        csv_data
    }
}
