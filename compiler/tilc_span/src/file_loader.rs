use std::{fs, io, path::Path};

pub trait FileLoader {
  fn file_exist(&self, path: &Path) -> bool;
  fn read_file(&self, path: &Path) -> io::Result<String>;
}

pub struct ActualFileLoader;
impl FileLoader for ActualFileLoader {
  fn file_exist(&self, path: &Path) -> bool {
    return path.exists();
  }

  fn read_file(&self, path: &Path) -> io::Result<String> {
    return fs::read_to_string(path);
  }
}
