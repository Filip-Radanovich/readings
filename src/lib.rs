use std::{default, io};

use serde::{Deserialize, Serialize};

const VERSION: Version = Version(0, 0, 1);
const OPTIONS: [&str; 4] = [
    "1. List previous entries",
    "2. Insert new entry",
    "3. Change location of manifest file",
    "4. Create new manifest",
];

#[derive(Default)]
pub struct Manifest<'a> {
    path: Option<&'a std::path::Path>,
    file: Option<std::fs::File>,
    manifest_file: Option<ManifestFile>,
}

impl<'a> Manifest<'a> {
    pub fn open(path: &'a str) -> Self {
        let path = std::path::Path::new(path);
        let file = std::fs::File::open(path);
        let manifest = Self {
            path: Some(path),
            ..Self::default()
        };
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_tdd() {
        Manifest::open("./manifest.venue");
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct ManifestFile {
    version: Version,
    entries: Vec<ManifestEntryFile>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
struct Version(u32, u32, u32);

/// YEAR-MONTH-DAY
#[derive(Default, Debug, Serialize, Deserialize)]
struct Date(u32, u32, u32);

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ManifestEntryFile {
    location: String,
    date: Date,
    power_meter_state: PowerMeterEntry,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct PowerMeterEntry {
    start: Option<PowerMeterState>,
    end: Option<PowerMeterState>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct PowerMeterState {
    code_108: f64,
    code_104: f64,
}

pub fn application(std_in: &io::Stdin, input: &mut String) {
    print_version();
    print_intro();
    print_options();
    loop {
        std_in
            .read_line(input)
            .expect("Error while reading standard input.");
        let parsed_input = input.trim().parse::<u8>();
        match parsed_input {
            Ok(num) => {
                println!();
                println!("{}", num);
                break;
            }
            Err(_) => {
                println!();
                println!("Only numbers allowed");
                input.clear();
                print_options();
            }
        }
    }
}

pub fn print_version() {
    println!("Version {}.{}.{}", VERSION.0, VERSION.1, VERSION.2);
}

pub fn print_intro() {
    println!("Application for registering power meter readings.");
    println!("For help press h or ?");
}

pub fn print_options() {
    for opt in OPTIONS.into_iter() {
        println!("{}", opt)
    }
}
