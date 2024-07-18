use std::{
    fs::File,
    io::{self, Read},
    path::{self, Path, PathBuf},
};

pub const DIR_SHADER: &str = "shader";

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

    pub fn get_in(&self, subdir: &str, file: impl AsRef<str>) -> Resource {
        let file: &str = file.as_ref().into();

        assert!(!file.contains("/") && !file.contains(path::MAIN_SEPARATOR));

        self.get(subdir.to_owned() + file)
    }
}

impl<'a> Resource<'a> {
    pub fn open(&'a mut self) -> io::Result<&'a File> {
        if self.file.is_none() {
            let file = File::open(self.root.join(&self.location).as_path())?;
            self.file = Some(file);
        }

        Ok(self.file.as_ref().unwrap())
    }

    pub fn read_to_str(&'a mut self) -> io::Result<String> {
        let mut file = self.open()?;

        let mut source = String::new();
        file.read_to_string(&mut source)?;

        Ok(source)
    }
}
