use std::{
  hash::{DefaultHasher, Hash, Hasher},
  io,
  path::Path,
  rc::Rc,
};

use parking_lot::RwLock;

use tilc_data_structure::{Hash64, UnHashMap};
use tilc_error::FatalError;

use crate::{BytePos, FileName, Pos, SandyqId};


pub struct TooLargeFileError;

#[derive(Debug)]
pub struct SourceMap {
  pub files: RwLock<SourceMapFiles>,
}
impl SourceMap {
  pub fn new() -> Self {
    return Self {
      files: RwLock::new(SourceMapFiles::default()),
    };
  }

  pub fn load_file(&self, path: &Path) -> io::Result<Rc<SourceFile>> {
    let source = std::fs::read_to_string(path)?;
    let filename =
      FileName::Real(crate::RealFileName::Local(path.to_path_buf()));

    return Ok(self.new_source_file(filename, source));
  }
  fn new_source_file(
    &self,
    filename: FileName,
    source: String,
  ) -> Rc<SourceFile> {
    return match self.try_new_source_file(filename, source) {
      Ok(source_file) => source_file,

      Err(TooLargeFileError) => FatalError::raise(),
    };
  }
  fn try_new_source_file(
    &self,
    filename: FileName,
    source: String,
  ) -> Result<Rc<SourceFile>, TooLargeFileError> {
    let stable_id = SourceFileHashId::from_filename(&filename);


    return match self.get_source_file_from_stable_id(&stable_id) {
      Some(source_file) => Ok(source_file),
      None => {
        let source_file = SourceFile::new(filename, source)?;
        return self.register_source_file(stable_id, source_file);
      }
    };
  }
  fn get_source_file_from_stable_id(
    &self,
    stable_id: &SourceFileHashId,
  ) -> Option<Rc<SourceFile>> {
    return self.files.read().files_hashmap.get(stable_id).cloned();
  }
  fn register_source_file(
    &self,
    stable_id: SourceFileHashId,
    mut source_file: SourceFile,
  ) -> Result<Rc<SourceFile>, TooLargeFileError> {
    let mut files = self.files.write();

    source_file.start_pos =
      BytePos::from_u32(if let Some(last_file) = files.files.last() {
        last_file
          .file_end_pos()
          .0
          .checked_add(1)
          .ok_or_else(|| TooLargeFileError)?
      } else {
        0
      });

    let source_file = Rc::new(source_file);
    files.files.push(source_file.clone());
    files.files_hashmap.insert(stable_id, source_file.clone());

    return Ok(source_file);
  }
}

#[derive(Debug, Default)]
pub struct SourceMapFiles {
  pub files: Vec<Rc<SourceFile>>,

  pub files_hashmap: UnHashMap<SourceFileHashId, Rc<SourceFile>>,
}


#[derive(Debug)]
pub struct SourceFile {
  pub name: FileName,
  pub source: Rc<str>,

  pub start_pos: BytePos,
  pub source_len: u32,

  pub id: SourceFileHashId,
}
impl SourceFile {
  pub fn new(
    name: FileName,
    source: String,
  ) -> Result<Self, TooLargeFileError> {
    let source_len =
      u32::try_from(source.len()).map_err(|_| TooLargeFileError)?;
    let id = SourceFileHashId::from_filename(&name);

    return Ok(Self {
      name,
      source: Rc::from(source),

      start_pos: BytePos::from_u32(0),
      source_len,

      id,
    });
  }

  fn file_end_pos(&self) -> BytePos {
    return self.abosolute_pos(self.source_len.into());
  }
  fn abosolute_pos(&self, pos: BytePos) -> BytePos {
    return self.start_pos + pos;
  }
}


#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SourceFileHashId(Hash64);
impl SourceFileHashId {
  pub fn from_filename(filename: &FileName) -> Self {
    return Self::from_filename_and_sandyq(filename, None);
  }
  pub fn from_filename_and_sandyq(
    filename: &FileName,
    sandyq_id: Option<SandyqId>,
  ) -> Self {
    let mut hasher = DefaultHasher::new();

    filename.hash(&mut hasher);
    sandyq_id.hash(&mut hasher);


    return Self(Hash64::new(hasher.finish()));
  }
}
