use std::{
    fs::File,
    io::{self, Read},
    path::{self, Path, PathBuf},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("couldn't convert std::path::Path to &str as it contains invalid UTF-8 characters")]
    PathInvalidUtf8,
}

pub struct ResourceManager {
    path: PathBuf,
}

pub struct Resource<'man> {
    root: &'man Path,
    location: PathBuf,
    file: Option<File>,
}

impl ResourceManager {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn get(&self, location: impl AsRef<str>) -> Resource {
        Resource {
            root: self.path.as_path(),
            location: location.as_ref().into(),
            file: None,
        }
    }

    pub fn get_from_path(&self, location: &Path) -> Result<Resource, Error> {
        Ok(self.get(location.to_str().ok_or(Error::PathInvalidUtf8)?))
    }

    pub fn get_in(&self, subdir: &str, file: impl AsRef<str>) -> Resource {
        let file: &str = file.as_ref().into();

        assert!(!file.contains("/") && !file.contains(path::MAIN_SEPARATOR));

        self.get(subdir.to_owned() + file)
    }
}

impl<'a> Resource<'a> {
    pub fn open(&'a mut self) -> Result<&'a File, Error> {
        if self.file.is_none() {
            let file = File::open(self.root.join(&self.location).as_path())?;
            self.file = Some(file);
        }

        Ok(self.file.as_ref().unwrap())
    }

    pub fn read_to_str(&'a mut self) -> Result<String, Error> {
        let mut file = self.open()?;

        let mut source = String::new();
        file.read_to_string(&mut source)?;

        Ok(source)
    }
}
