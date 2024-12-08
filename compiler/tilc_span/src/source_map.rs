use std::{
  cell::{RefCell, RefMut},
  collections::HashMap,
  hash::{DefaultHasher, Hash, Hasher},
  io,
  path::{Path, PathBuf},
  rc::Rc,
};

use md5::{Digest, Md5};

use tilc_data_structures::{Hash64, UnidirectionalVec};

use crate::{ActualFileLoader, BytePos, FileLoader, Pos, StablePackageId};


/// If content of the file exceeds u32 4gb (4000000000 chars assuming 1 char = 1 byte)
#[derive(Debug)]
pub struct TooLargeFileError;


#[derive(Debug)]
pub struct SourceMap {
  pub files: RefCell<SourceMapFiles>,
  pub file_loader: Box<dyn FileLoader>,

  // pub file_mapping: ,

  // NOTE: Ignored for now
  pub(crate) _hash_algorithm: SourceFileHashAlgorithm,
}
impl SourceMap {
  pub fn new() -> Self {
    return Self {
      files: RefCell::new(Default::default()),

      file_loader: Box::new(ActualFileLoader),

      _hash_algorithm: Default::default(),
    };
  }


  pub fn load_file(&self, path: &Path) -> io::Result<Rc<SourceFile>> {
    let source: String = self.file_loader.read_file(path)?;
    let filename: FileName = path.to_path_buf().into();


    return Ok(self.new_source_file(filename, source).unwrap());
  }


  pub fn new_source_file(
    &self,
    filename: FileName,
    source: String,
  ) -> Result<Rc<SourceFile>, TooLargeFileError> {
    let stable_id: SourceFileHashId =
      SourceFileHashId::from_filename(&filename);

    return match self.get_source_file_from_stable_id(&stable_id) {
      Some(source_file) => Ok(source_file),

      None => {
        let source_file = SourceFile::new(
          filename,
          source,
          self._hash_algorithm,
          Some(self._hash_algorithm),
        )?;
        return self.registre_source_file(stable_id, source_file);
      }
    };
  }


  pub fn registre_source_file(
    &self,
    stable_id: SourceFileHashId,
    mut soruce_file: SourceFile,
  ) -> Result<Rc<SourceFile>, TooLargeFileError> {
    let mut files: RefMut<'_, SourceMapFiles> = self.files.borrow_mut();

    soruce_file.start_pos =
      BytePos(if let Some(last_file) = files.files.last() {
        last_file
          .file_end_pos()
          .0
          .checked_add(1)
          .ok_or(TooLargeFileError)?
      } else {
        0
      });

    let file: Rc<SourceFile> = Rc::new(soruce_file);
    files.files.push(file.clone());
    files.files_hashmap.insert(stable_id, file.clone());
    return Ok(file);
  }
  pub fn get_source_file_from_stable_id(
    &self,
    stable_id: &SourceFileHashId,
  ) -> Option<Rc<SourceFile>> {
    return self.files.borrow().files_hashmap.get(stable_id).cloned();
  }
}
#[derive(Default, Debug)]
pub struct SourceMapFiles {
  // NOTE: This vector only growth, it cannot delete items in it
  pub files: UnidirectionalVec<Rc<SourceFile>>,

  pub files_hashmap: HashMap<SourceFileHashId, Rc<SourceFile>>,
}
#[derive(Debug, Clone)]
pub struct SourceFile {
  pub name: FileName,
  pub source: Rc<String>,

  pub src_hash: SourceFileHash,
  /// For the package manager to check whether the external package is damaged or new/old
  pub checksum_hash: Option<SourceFileHash>, // TODO: Implement this feature

  pub start_pos: BytePos,
  pub source_len: u32,

  pub id: SourceFileHashId,
}
impl SourceFile {
  /// TODO: Put proper error
  pub fn new(
    name: FileName,
    source: String,
    hash_kind: SourceFileHashAlgorithm,
    checksum_hash_kind: Option<SourceFileHashAlgorithm>,
  ) -> Result<Self, TooLargeFileError> {
    let src_hash: SourceFileHash =
      SourceFileHash::new(source.as_bytes(), hash_kind);
    let checksum_hash: Option<SourceFileHash> =
      checksum_hash_kind.map(|checksum_hash_kind: SourceFileHashAlgorithm| {
        if checksum_hash_kind == hash_kind {
          return src_hash;
        } else {
          return SourceFileHash::new(source.as_bytes(), checksum_hash_kind);
        };
      });

    let source_len: u32 =
      u32::try_from(source.len()).map_err(|_| TooLargeFileError)?;
    let id: SourceFileHashId = SourceFileHashId::from_filename(&name);

    return Ok(Self {
      name,
      source: Rc::new(source),

      src_hash,
      checksum_hash,

      start_pos: BytePos(0),
      source_len,

      id,
    });
  }


  pub fn file_end_pos(&self) -> BytePos {
    return self.abosolute_pos(self.source_len.into());
  }
  pub fn abosolute_pos(&self, pos: BytePos) -> BytePos {
    return BytePos::from_u32(self.start_pos.to_u32() + pos.to_u32());
  }
}

#[derive(Clone, Copy, Debug, Hash)]
pub struct SourceFileHash {
  hash_kind: SourceFileHashAlgorithm,
  value: [u8; 32],
}
impl SourceFileHash {
  pub fn new(bytes: &[u8], hash_kind: SourceFileHashAlgorithm) -> Self {
    let mut hash: SourceFileHash = SourceFileHash {
      hash_kind,
      value: Default::default(),
    };
    let len: usize = hash.hash_byte_len();
    let data: &[u8] = bytes.as_ref();
    let value: &mut [u8] = &mut hash.value[..len];

    match hash_kind {
      SourceFileHashAlgorithm::Md5 => {
        value.copy_from_slice(&Md5::digest(data));
      }
    };


    return hash;
  }


  fn hash_byte_len(&self) -> usize {
    return match self.hash_kind {
      SourceFileHashAlgorithm::Md5 => 16,
    };
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SourceFileHashId(Hash64);
impl SourceFileHashId {
  pub fn from_filename(filename: &FileName) -> Self {
    return Self::from_filename_and_package(filename, None);
  }

  pub fn from_filename_and_package(
    filename: &FileName,
    package: Option<StablePackageId>,
  ) -> Self {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    filename.hash(&mut hasher);
    package.hash(&mut hasher);


    return Self(Hash64::new(hasher.finish()));
  }
}
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum SourceFileHashAlgorithm {
  Md5,
}
impl Default for SourceFileHashAlgorithm {
  fn default() -> Self {
    return Self::Md5;
  }
}


pub struct FilePathMapping {
  mapping: Vec<(PathBuf, PathBuf)>,
}
impl FilePathMapping {
  pub const fn empty() -> Self {
    return Self {
      mapping: Vec::new(),
    };
  }

  pub fn new(mapping: Vec<(PathBuf, PathBuf)>) -> Self {
    return Self { mapping };
  }
}


#[derive(Clone, Debug, Hash)]
pub enum FileName {
  Real(RealFileName),

  // TODO
  CommandLineInput,
}
impl From<PathBuf> for FileName {
  fn from(path: PathBuf) -> Self {
    return Self::Real(RealFileName::Local(path));
  }
}
#[derive(Clone, Debug, Hash)]
pub enum RealFileName {
  Local(PathBuf),

  Remapped(PathBuf, PathBuf),
}
