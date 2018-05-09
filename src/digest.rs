use csv;
use csv::StringRecord;
use std;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::hash::Hasher;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use twox_hash::XxHash;

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
        match record {
            Ok(string_record) => {
                let record = string_record.get(0).unwrap().to_owned();
                hash_sender.send((String::from(record), string_record)).unwrap();
            }
            _ => ()
        };
    });

    drop(hash_sender);
    hash = hash_handler.join().unwrap();

    let (hash_sender, hash_receiver): (Sender<(String, StringRecord)>, Receiver<(String, StringRecord)>) = channel();

    let hash_handler = thread::spawn(move || {
        for (key, value) in hash_receiver.iter() {
            match hash.get(&key) {
                Some(existing_record) => match existing_record.eq(&get_hash(value.as_slice().as_bytes())) {
                    true => (),
                    false => println!("Changed {:?}", value)
                },
                None => println!("Addition {:?}", value)
            }
        };
    });

    for record in another_reader.records() {
        match record {
            Ok(string_record) => {
                let record = string_record.get(0).unwrap().to_owned();
                hash_sender.send((String::from(record), string_record)).unwrap();
            }
            Err(_e) => {}
        }
    }
    drop(hash_sender);
    hash_handler.join().unwrap();
}

mod test {}