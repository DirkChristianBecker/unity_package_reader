use flate2::read::GzDecoder;
use rust_tools::prelude::*;
use std::path::PathBuf;
use tar::Archive;

pub enum UnityPackageReaderError {
    PackageNotFound,
    CorruptPackage,
    DirectoryCouldNotBeCreated,
}

pub struct UnityPackage {
    path: PathBuf,
}

impl UnityPackage {
    pub fn new(file_name: &str) -> Self {
        let name = String::from(file_name);
        let mut path = Self::get_assets_dir();
        path.push(name);

        UnityPackage { path }
    }

    fn get_tmp_dir() -> PathBuf {
        let mut tmp_path = std::env::current_dir().unwrap();
        tmp_path.push("src");
        tmp_path.push("tmp");

        tmp_path
    }

    fn get_assets_dir() -> PathBuf {
        let mut p = std::env::current_dir().unwrap();
        p.push("src");
        p.push("assets");

        p
    }

    pub fn unpack_package(&self) -> Result<PathBuf, UnityPackageReaderError> {
        let tmp = get_file_as_byte_vec(&self.path);
        match tmp {
            Ok(bytes) => {
                let temp = Self::get_tmp_dir();

                let tar = GzDecoder::new(&bytes[..]);
                let mut archive = Archive::new(tar);

                match std::fs::create_dir_all("/some/dir") {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(UnityPackageReaderError::DirectoryCouldNotBeCreated);
                    }
                }

                match archive.unpack(&temp) {
                    Ok(_) => Ok(temp),
                    Err(_) => Err(UnityPackageReaderError::CorruptPackage),
                }
            }
            Err(e) => match e {
                FileErrors::FileNotFound => Err(UnityPackageReaderError::PackageNotFound),
                FileErrors::CorruptFile => Err(UnityPackageReaderError::CorruptPackage),
            },
        }
    }
}
