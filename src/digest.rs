use csv;
use std;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::hash::Hasher;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use twox_hash::XxHash;
use csv::StringRecord;

pub fn digest(mut reader: csv::Reader<std::fs::File>, mut another_reader: csv::Reader<std::fs::File>) {
    fn get_hash(input: &[u8]) -> u64 {
        let mut hasher = XxHash::with_seed(0);
        hasher.write(input);
        hasher.finish()
    }
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash>> = Default::default();
    let (hash_sender, hash_receiver): (Sender<(String, StringRecord)>, Receiver<(String, StringRecord)>) = channel();

    let hash_handler = thread::spawn(move || {
        for (key, value) in hash_receiver.iter() {
            hash.insert(key, get_hash(value.as_slice().as_bytes()));
        };
        hash
    });

    reader.records().for_each(|record| {
        let hash_sender_clone = hash_sender.clone();
        match record {
            Ok(string_record) => {
                let
                record_clone = string_record.clone();
                if let Some(record) = record_clone.get(0) {
                    hash_sender_clone.send((String::from(record), string_record)).unwrap();
                }
            }
            _ => ()
        };
    });

    drop(hash_sender);
    hash = hash_handler.join().unwrap();

    for record in another_reader.records() {
        match record {
            Ok(string_record) => {
                let record_clone = string_record.clone();
                if let Some(record) = record_clone.get(0) {
                    match hash.get(&String::from(record)) {
                        Some(record_value) => {
                            match get_hash(string_record.as_slice().as_bytes()).cmp(record_value) {
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
            Err(_e) => {}
        }
    }
}

mod test {}