use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};


use tilc_data_structures::{Hash128, UnidirectionalVec};


use crate::{ActualFileLoader, FileLoader};


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
}
#[derive(Default)]
pub struct SourceMapFiles {
  // NOTE: This vector only growth, it cannot delete items in it
  pub files: UnidirectionalVec<SourceFile>,

  pub files_hashmap: HashMap<SourceFileHashId, SourceFile>,
}
pub struct SourceFile {
  pub name: PathBuf,
  pub src: Rc<String>,

  /// For the package manager to check whether the external package is damaged or new/old
  pub _checksum_hash: bool, // TODO: Implement this feature
}


pub struct SourceFileHashId(Hash128);
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
