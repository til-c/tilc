use std::{path::Path, rc::Rc};

use tilc_advanced_lexer::TokenReader;
use tilc_ast::TokenStream;
use tilc_lexer::Lexer;
use tilc_parser::Parser;
use tilc_session::ParseSession;
use tilc_span::{BytePos, ErrorGuaranteed, SourceFile};

use crate::token_trees::TokenTreesReader;

mod token_trees;


pub fn new_parser_from_file<'psess>(
  parse_session: &'psess ParseSession,
  path: &Path,
) -> Result<Parser<'psess>, ErrorGuaranteed> {
  let source_file = parse_session.source_map.load_file(path).unwrap();
  return new_parser_from_source_file(parse_session, source_file);
}
fn new_parser_from_source_file<'psess>(
  parse_session: &'psess ParseSession,
  source_file: Rc<SourceFile>,
) -> Result<Parser<'psess>, ErrorGuaranteed> {
  let token_stream = source_file_to_stream(parse_session, source_file)?;
  let parser = Parser::new(parse_session, token_stream);

  return Ok(parser);
}
fn source_file_to_stream<'psess>(
  parse_session: &'psess ParseSession,
  source_file: Rc<SourceFile>,
) -> Result<TokenStream, ErrorGuaranteed> {
  let source = source_file.source.as_ref();

  return lex_token_stream(&parse_session, source, source_file.start_pos);
}

fn lex_token_stream<'psess, 'src>(
  parse_session: &'psess ParseSession,
  src: &'src str,
  start_pos: BytePos,
) -> Result<TokenStream, ErrorGuaranteed> {
  let lexer = Lexer::new(src);
  let token_reader =
    TokenReader::new(src, lexer, parse_session, start_pos, start_pos);

  let (stream, err) = lex_all_token_trees(token_reader);
  return match err {
    Ok(_) => Ok(stream),

    Err(_) => panic!(),
  };
}
fn lex_all_token_trees(
  token_reader: TokenReader,
) -> (TokenStream, Result<(), ErrorGuaranteed>) {
  let mut ttr = TokenTreesReader::new(token_reader);

  let (_, stream, err) = ttr.lex_token_trees(false);
  return (stream, err);
}
