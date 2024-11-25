use std::{
  cell::RefCell,
  collections::HashMap,
  hash::{DefaultHasher, Hash, Hasher},
  io,
  path::{Path, PathBuf},
  rc::Rc,
};

use md5::{Digest, Md5};

use tilc_data_structures::{Hash64, UnidirectionalVec};

use crate::{ActualFileLoader, FileLoader, Pos, StablePackageId};


pub struct SourceMap {
  pub files: RefCell<SourceMapFiles>,
  pub file_loader: Box<dyn FileLoader>,

  // pub file_mapping: ,

  // NOTE: Ignored for now
  _hash_algorithm: SourceFileHashAlgorithm,
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


    todo!()
  }
}
#[derive(Default)]
pub struct SourceMapFiles {
  // NOTE: This vector only growth, it cannot delete items in it
  pub files: UnidirectionalVec<SourceFile>,

  pub files_hashmap: HashMap<SourceFileHashId, SourceFile>,
}
pub struct SourceFile {
  pub name: FileName,
  pub source: Rc<String>,

  pub src_hash: SourceFileHash,
  /// For the package manager to check whether the external package is damaged or new/old
  pub checksum_hash: Option<SourceFileHash>, // TODO: Implement this feature

  pub start_pos: Pos,
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
  ) -> Result<Self, ()> {
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

    let source_len: u32 = u32::try_from(source.len()).map_err(|_| ())?;
    let id: SourceFileHashId = SourceFileHashId::from_filename(&name);

    return Ok(Self {
      name,
      source: Rc::new(source),

      src_hash,
      checksum_hash,

      start_pos: Pos(0),
      source_len,

      id,
    });
  }
}

#[derive(Clone, Copy, Hash)]
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
#[derive(Clone, Copy, Hash, PartialEq)]
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


#[derive(Hash)]
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
#[derive(Hash)]
pub enum RealFileName {
  Local(PathBuf),

  Remapped(PathBuf, PathBuf),
}
