mod token_trees;


use std::{path::Path, rc::Rc};

use tilc_advanced_lexer::TokenReader;
use tilc_ast::TokenStream;
use tilc_lexer::Lexer;
use tilc_parser::Parser;
use tilc_session::ParseSession;
use tilc_span::{ErrorGuaranteed, ModuleIdx, Pos, SourceFile};
use token_trees::TokenTreesReader;


fn new_parser_from_file<'psess>(
  parse_session: &'psess ParseSession,
  path: &Path,
) -> Result<Parser<'psess>, ErrorGuaranteed> {
  let source_file: Rc<SourceFile> =
    parse_session.source_map().load_file(path).unwrap();
  return new_parser_from_source_file(parse_session, source_file);
}
fn new_parser_from_source_file<'psess>(
  parse_session: &'psess ParseSession,
  source_file: Rc<SourceFile>,
) -> Result<Parser<'_>, ErrorGuaranteed> {
  let token_stream: TokenStream =
    source_file_to_stream(parse_session, source_file)?;
  let parser: Parser<'_> = Parser::new(parse_session, token_stream);

  return Ok(parser);
}
fn source_file_to_stream<'psess>(
  parse_session: &'psess ParseSession,
  source_file: Rc<SourceFile>,
) -> Result<TokenStream, ErrorGuaranteed> {
  let source: &String = source_file.source.as_ref();

  return lex_token_stream(&parse_session, source, source_file.start_pos);
}
/// TODO: Hanlde errors properly
pub fn lex_token_stream<'psess, 'src>(
  parse_session: &'psess ParseSession,
  src: &'src str,
  start_pos: Pos,
) -> Result<TokenStream, ErrorGuaranteed> {
  let lexer: Lexer = Lexer::new(src);
  let token_reader: TokenReader = TokenReader::new(
    src,
    lexer,
    parse_session,
    start_pos,
    start_pos,
    ModuleIdx::new(0), // FIXME: idk deal with it somehow
  );

  let (stream, err): (TokenStream, Result<(), ErrorGuaranteed>) =
    lex_all_token_trees(token_reader);
  return match err {
    Ok(_) => Ok(stream),

    Err(_) => panic!(),
  };
}
fn lex_all_token_trees(
  token_reader: TokenReader,
) -> (TokenStream, Result<(), ErrorGuaranteed>) {
  let mut ttr: TokenTreesReader = TokenTreesReader::new(token_reader);

  let (_, stream, err): (_, TokenStream, Result<(), ErrorGuaranteed>) =
    ttr.lex_token_trees(false);
  return (stream, err);
}
