use std::{env, fs, path::PathBuf, time::SystemTime};

use indicatif::{ProgressBar, ProgressStyle};
use redb::{Database, ReadableTable, TableDefinition};
use time::{Duration, OffsetDateTime};
use walkdir::{WalkDir, DirEntry};

use crate::hashing::{self, CsvData, save_csv};

const TABLE: TableDefinition<&str, u64> = TableDefinition::new("last_modified");

pub fn sniff(msg: String, path: Option<PathBuf>) {
    println!("Beagle🐶start sniff：");

    let path: PathBuf = match path {
        Some(p) => p,
        None => env::current_dir().unwrap(),
    };

    let mut init_path = path.clone();
    init_path.push(".file-diff");

    if init_path.exists() {
        let mut db_path: PathBuf = init_path.clone();
        db_path.push("record.redb");
        let db = Database::create(db_path).unwrap();

        let read_txn = db.begin_read().unwrap();
        let table = read_txn.open_table(TABLE).unwrap();

        let last_record = table.get("last_record").unwrap().unwrap().value();
        // println!("last_record: {:?}", last_record);

        let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
        let mut data: Vec<CsvData> = Vec::new();

        let walker = WalkDir::new(&path).into_iter();
        let total_count = walker.count();
        let pb = ProgressBar::new(total_count.try_into().unwrap());
        pb.set_style(spinner_style.clone());
        let walker = WalkDir::new(&path).into_iter();
        let mut count = 0;
        for file in walker
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok())
        {
            if file.metadata().unwrap().is_file() {
                let file_path: String = String::from(file.path().to_string_lossy());
                let metadata = fs::metadata(&file_path).unwrap();
                if let Ok(time) = metadata.modified() {
                    match time.duration_since(SystemTime::UNIX_EPOCH) {
                        Ok(n) => {
                            if n.as_secs() > last_record {
                                // println!("get hash from {} ", file_path);
                                pb.set_style(spinner_style.clone());
                                pb.set_message(format!("get hash from {} ", file_path));
                                pb.inc(1);
                                let duration = Duration::try_from(
                                    time.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
                                )
                                .unwrap();
                                let offset_date_time = OffsetDateTime::UNIX_EPOCH + duration;
                                let csv_data: CsvData = hashing::get_file_hash(file_path, offset_date_time.to_string());

                                data.push(csv_data);
                                count += 1;
                            }
                        }
                        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                    }
                } else {
                    println!("Not supported on this platform");
                }
            }
        }
        let now = SystemTime::now();
        match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                
                let write_txn = db.begin_write().unwrap();
                {
                    let mut table = write_txn.open_table(TABLE).unwrap();
                    table.insert("last_record", n.as_secs()).unwrap();
                }
                write_txn.commit().unwrap();
                init_path.push(format!("{}_{}", n.as_secs(), msg));
            }
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
        save_csv(init_path, data);
        pb.finish_with_message(format!("Done! Total File:{count}"));
    } else {
        println!("folder not exist");
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
