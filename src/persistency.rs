use serde::{Deserialize, Serialize};
use std::default;
use std::fs::File;
use std::io::{self};
use super::{
    Error,
    Version,
    PowerMeterInfo,
    PowerMeterEntry
};

pub struct Manifest<'a> {
    path: &'a std::path::Path,
    file: std::fs::File,
    manifest_file: Option<ManifestFile>,
}

impl<'a> Manifest<'a> {
    pub fn open(path: &'a str) -> Result<Self, Error> {
        let path = std::path::Path::new(path);
        let file = match std::fs::File::options().write(false).read(true).open(path) {
            Ok(file) => file,
            Err(error) => match error {
                _ => return Err(Error::File { path, error }),
            },
        };
        let (file, manifest) = ManifestFile::open(file)?;

        Ok(Self {
            path,
            file,
            manifest_file: Some(manifest),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ManifestError {
    DataCorrupted(),
    NotValidManifest,
}
// listovi su podaci, linije relacije
#[derive(Default, Debug, Serialize, Deserialize)]
struct ManifestFile {
    version: Version,
    entries: Entries,
}

impl ManifestFile {
    pub fn open<'a>(file: File) -> Result<(File, ManifestFile), Error<'a>> {
        todo!()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Entries {
    power_meters: Option<Vec<PowerMeterInfo>>,
    power_readings: Option<Vec<PowerMeterEntry>>
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn manifest_tdd() {
        let mut file_name = PathBuf::from("./test_files/manifest.venue");
        let file_name = file_name.to_str();
        match file_name {
            Some(name) => {
                Manifest::open(name).unwrap();
            }
            None => panic!("File path coud not be read."),
        }
    }
}
