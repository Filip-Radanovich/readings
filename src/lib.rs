use std::default;
use std::fs::File;
use std::io::{self};

use serde::{Deserialize, Serialize};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
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
    pub fn open(path: &'a str) -> Result<Self, Error>  {
        let path = std::path::Path::new(path);
        let file = match std::fs::File::open(path){
            Ok(file) => file,
            Err(e) => {
                match e {
                    _ => return Err(Error::FileNotFound)
                }
            }
        };
        let manifest = ManifestFile::open(&file)?;
        Ok(Self {
            path: Some(path),
            file: Some(file),
            manifest_file: Some(manifest)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn manifest_tdd() {
        let mut file_name = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_name.push("/test_files/manifest.venue");
        let file_name = file_name.to_str();
        match file_name {
            Some(name) => {Manifest::open(name).unwrap();},
            None => panic!("File path coud not be read.")
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct ManifestFile {
    version: Version,
    entries: Vec<ManifestEntryFile>,
}

impl ManifestFile{
    pub fn open(file: &File) -> Result<ManifestFile, Error> {
        todo!()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    FileNotFound,
    DataInvalid,

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
    start: PowerMeterState,
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
    println!("Version {}", built_info::PKG_VERSION);
}

pub fn print_intro() {
    println!("{}", built_info::PKG_DESCRIPTION);
    println!("For help press h or ?");
}

pub fn print_options() {
    for opt in OPTIONS.into_iter() {
        println!("{}", opt)
    }
}
