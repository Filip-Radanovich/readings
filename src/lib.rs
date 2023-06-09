mod persistency;
use persistency::*;
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


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ReadingEntry {
    power_meter_info: Option<PowerMeterInfo>,
    power_meter_state: PowerMeterEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PowerMeterInfo {
    Fixed(Address, PowerMeterNumber),
    Portable(Option<Description>, PowerMeterNumber),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    location: (String, Option<u32>),
}

impl Address {
    /// Just checks that name of location isn't too big.
    /// Its currently set to lenght of hundred but could be bigger in the future.
    pub fn validate<'a>(location: (String, Option<u32>)) -> Result<Address, Error<'a>> {
        match location.0.len() {
            1..=100 => Ok(Address { location }),
            0 => Err(Error::AddressTooShort),
            _ => Err(Error::AdrressTooLong),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerMeterNumber;
#[derive(Debug, Serialize, Deserialize)]
pub struct Description;


#[derive(Debug)]
pub enum Error<'a> {
    File {
        path: &'a std::path::Path,
        error: std::io::Error,
    },
    Manifest {
        path: &'a std::path::Path,
        file: std::fs::File,
        error: ManifestError,
    },
    AddressTooShort,
    AdrressTooLong,
}
#[derive(Default, Debug, Serialize, Deserialize)]
struct Version(u32, u32, u32);

/// YEAR-MONTH-DAY
#[derive(Default, Debug, Serialize, Deserialize)]
struct Date(u32, u32, u32);



pub fn application(
    std_in: &io::Stdin,
    input: &mut String,
) {
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
