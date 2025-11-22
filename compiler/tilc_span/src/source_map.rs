use std::{
  fs::{self, File},
  hash::{BuildHasherDefault, DefaultHasher, Hash, Hasher},
  io,
  path::Path,
  rc::Rc,
};

use parking_lot::RwLock;

use tilc_data_structures::{Hash64, UnHashMap, Unhasher};

use crate::{BytePos, Filename, Pos, RealFileName, SandyqId};

struct TooLargeFileError;

#[derive(Debug)]
pub struct SourceMap {
  files: RwLock<SourceMapFiles>,
}
impl SourceMap {
  pub const fn new() -> Self {
    Self {
      files: RwLock::new(SourceMapFiles {
        files: Vec::new(),
        files_by_hash_id: UnHashMap::with_hasher(BuildHasherDefault::new()),
      }),
    }
  }

  pub fn load_file(&self, path: &Path) -> io::Result<Rc<SourceFile>> {
    let source = fs::read_to_string(path)?;
    let filename = Filename::Real(RealFileName::Local(path.to_path_buf()));

    Ok(self.new_source_file(filename, source))
  }

  fn new_source_file(&self, filename: Filename, source: String) -> Rc<SourceFile> {
    match self.try_new_source_file(filename, source) {
      Ok(source_file) => source_file,
      Err(TooLargeFileError) => panic!(),
    }
  }

  fn try_new_source_file(
    &self,
    filename: Filename,
    source: String,
  ) -> Result<Rc<SourceFile>, TooLargeFileError> {
    let id = SourceFileHashId::from_filename(&filename);

    match self.get_source_file_from_id(&id) {
      Some(source_file) => Ok(source_file),
      None => {
        let source_file = SourceFile::new(filename, source)?;
        self.register_source_file(id, source_file)
      }
    }
  }

  fn get_source_file_from_id(&self, id: &SourceFileHashId) -> Option<Rc<SourceFile>> {
    self.files.read().files_by_hash_id.get(id).cloned()
  }
  fn register_source_file(
    &self,
    id: SourceFileHashId,
    mut source_file: SourceFile,
  ) -> Result<Rc<SourceFile>, TooLargeFileError> {
    let mut files = self.files.write();

    source_file.start_pos = BytePos::from_u32(if let Some(last_file) = files.files.last() {
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
    files.files_by_hash_id.insert(id, source_file.clone());

    Ok(source_file)
  }
}

#[derive(Debug)]
struct SourceMapFiles {
  files: Vec<Rc<SourceFile>>,
  files_by_hash_id: UnHashMap<SourceFileHashId, Rc<SourceFile>>,
}

#[derive(Debug)]
pub struct SourceFile {
  src: Rc<str>,

  start_pos: BytePos,
  src_len: u32,

  id: SourceFileHashId,
}
impl SourceFile {
  fn new(filename: Filename, source: String) -> Result<Self, TooLargeFileError> {
    let src_len = u32::try_from(source.len()).map_err(|_| TooLargeFileError)?;
    let id = SourceFileHashId::from_filename(&filename);

    Ok(Self {
      src: Rc::from(source),

      start_pos: BytePos::from_u32(0),
      src_len,

      id,
    })
  }

  pub fn src(&self) -> &str {
    self.src.as_ref()
  }
  pub fn start(&self) -> BytePos {
    self.start_pos
  }

  fn file_end_pos(&self) -> BytePos {
    self.abosolute_pos(self.src_len.into())
  }
  fn abosolute_pos(&self, pos: BytePos) -> BytePos {
    self.start_pos + pos
  }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
struct SourceFileHashId(Hash64);
impl SourceFileHashId {
  fn from_filename(filename: &Filename) -> Self {
    Self::from_filename_and_sandyq(filename, None)
  }

  fn from_filename_and_sandyq(filename: &Filename, sandyq_id: Option<SandyqId>) -> Self {
    let mut hasher = DefaultHasher::new();

    filename.hash(&mut hasher);
    sandyq_id.hash(&mut hasher);

    Self(Hash64::new(hasher.finish()))
  }
}
