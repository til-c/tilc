use std::{path::Path, rc::Rc};

use tilc_advanced_lexer::TokenReader;
use tilc_ast::TokenStream;
use tilc_error::ErrorGuaranteed;
use tilc_lexer::Lexer;
use tilc_parser::Parser;
use tilc_session::ParseSession;
use tilc_span::{BytePos, SourceFile};

use crate::TokenTreesReader;

pub fn new_parser_from_file<'psess>(
  psess: &'psess ParseSession,
  path: &Path,
) -> Result<Parser<'psess>, ErrorGuaranteed> {
  let source_file = psess.source_map().load_file(path).unwrap();
  new_source_from_source_file(psess, source_file)
}
fn new_source_from_source_file<'psess>(
  psess: &'psess ParseSession,
  source_file: Rc<SourceFile>,
) -> Result<Parser<'psess>, ErrorGuaranteed> {
  let token_stream = source_file_to_stream(psess, source_file)?;
  let parser = Parser::new(psess, token_stream);
  Ok(parser)
}

fn source_file_to_stream<'psess>(
  psess: &'psess ParseSession,
  source_file: Rc<SourceFile>,
) -> Result<TokenStream, ErrorGuaranteed> {
  let src = source_file.src();

  lex_token_stream(&psess, src, source_file.start())
}

fn lex_token_stream<'psess, 'src>(
  psess: &'psess ParseSession,
  src: &'src str,
  start_pos: BytePos,
) -> Result<TokenStream, ErrorGuaranteed> {
  let lexer = Lexer::new(src);
  let token_reader = TokenReader::new(src, lexer, psess, start_pos, start_pos);

  let (stream, err) = lex_all_token_trees(token_reader);
  match err {
    Ok(_) => Ok(stream),
    Err(_) => panic!(),
  }
}
fn lex_all_token_trees(token_reader: TokenReader) -> (TokenStream, Result<(), ErrorGuaranteed>) {
  let mut ttr = TokenTreesReader::new(token_reader);

  let (_, stream, err) = ttr.lex_token_trees(false);
  (stream, err)
}
