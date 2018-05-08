extern crate clap;
extern crate csv;
extern crate twox_hash;
extern crate chan;

use clap::{App, Arg};
use std::fs::File;

mod digest;

fn main() {
    let args = App::new("CsvDiff")
        .version("0.1")
        .author("Sathish <tsatiz@gmail.com>, Aswin Karthik")
        .about("Compares two csv files and returns missing/modified rows")
        .arg(Arg::with_name("first_file")
            .help("Sets the first file to compare")
            .index(1).required(true)
        )
        .arg(Arg::with_name("second_file")
            .help("Sets the first file to compare")
            .index(2).required(true))
        .get_matches();

    if let (Some(first_file), Some(second_file)) = (args.value_of("first_file"), args.value_of("second_file")) {
        println!("Value for first_file: {}", first_file);
        println!("Value for second_file: {}", second_file);
//        let mut reader1 =ReaderBuilder::new()
//            .delimiter(b'|')
//            .from_reader(File::open(first_file)).unwrap();
//        let mut reader2 =ReaderBuilder::new()
//            .delimiter(b'|')
//            .from_reader(File::open(second_file)).unwrap();
        let mut reader1 = csv::Reader::from_reader(File::open(first_file).unwrap());
        let mut reader2 = csv::Reader::from_reader(File::open(second_file).unwrap());

        digest::digest(reader1, reader2);
//        let mut rdr2 = csv::Reader::from_reader(File::open(second_file));
    }
}
