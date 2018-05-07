use csv;
use std;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::hash::Hasher;
use twox_hash::XxHash;

pub fn digest(mut reader: csv::Reader<std::fs::File>, mut another_reader: csv::Reader<std::fs::File>) {

//    hash.insert(42, "the answer");
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash>> = Default::default();
    for record in reader.records() {
        match record {
            Ok(string_record) => {
                let record_clone = string_record.clone();
                if let Some(record) = record_clone.get(0) {
                    hash.insert(String::from(record), get_hash(format!("{:?}", string_record).as_bytes()));
                }
            }
            Err(e) => {}
        }
    }

    fn get_hash(input: &[u8]) -> u64 {
        let mut hasher = XxHash::with_seed(0);
        hasher.write(input);
        hasher.finish()
    }

    for record in another_reader.records() {
        match record {
            Ok(string_record) => {
                let record_clone = string_record.clone();
                if let Some(record) = record_clone.get(0) {
                    match hash.get(&String::from(record)) {
                        Some(record_value) => {
                            match get_hash(format!("{:?}", string_record).as_bytes()).cmp(record_value) {
                                Ordering::Equal => (),
                                _ => println!("Changed {:?}", string_record)
                            }
                        }
                        None => {
                            println!("Addition {:?}", string_record);
                        }
                    }
                }
            }
            Err(e) => {}
        }
    }
}

mod test {}