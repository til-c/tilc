use std::{cell::RefCell, rc::Rc};

use crate::{BytePos, FileName, Pos};


pub struct TooLargeFileError;

#[derive(Debug)]
pub struct SourceMap {
  pub files: RefCell<SourceMapFiles>,
}
impl SourceMap {
  pub fn new() -> Self {
    return Self {
      files: RefCell::new(SourceMapFiles::default()),
    };
  }
}

#[derive(Debug, Default)]
pub struct SourceMapFiles {
  files: Vec<Rc<SourceFile>>,
}


#[derive(Debug)]
pub struct SourceFile {
  pub name: FileName,
  pub source: Rc<str>,

  pub start_pos: BytePos,
  pub source_len: u32,
}
impl SourceFile {
  pub fn new(
    name: FileName,
    source: String,
  ) -> Result<Self, TooLargeFileError> {
    let source_len =
      u32::try_from(source.len()).map_err(|_| TooLargeFileError)?;

    return Ok(Self {
      name,
      source: Rc::from(source),

      start_pos: BytePos::from_u32(0),
      source_len,
    });
  }
}
