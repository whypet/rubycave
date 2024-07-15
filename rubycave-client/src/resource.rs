use std::{
    fs::File,
    path::{Path, PathBuf},
};

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

    pub fn get(&self, subdir: &str) -> Resource {
        Resource {
            root: self.path.as_path(),
            location: subdir.into(),
            file: None,
        }
    }
}

impl<'a> Resource<'a> {
    pub fn open(&'a mut self) -> std::io::Result<&'a File> {
        if self.file.is_none() {
            let file = File::open(&self.location.join(self.root).as_path())?;
            self.file = Some(file);
        }

        Ok(self.file.as_ref().unwrap())
    }
}
