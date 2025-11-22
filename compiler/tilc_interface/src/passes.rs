use std::path::PathBuf;

use tilc_ast::Sandyq;
use tilc_error::FatalError;
use tilc_parse::new_parser_from_file;
use tilc_session::{Input, ParseSession, Session};
use tilc_span::with_session_globals;

use crate::Result;

pub fn runner(args: &[String]) {
  let input_file = Input::File(match args.get(0) {
    Some(path) => PathBuf::from(path),
    None => FatalError.raise(),
  });
  let source_map = with_session_globals(|session_globals| session_globals.source_map());
  let psess = ParseSession::new(source_map);
  let session = Session { input_file, psess };

  let sandyq = parse(&session);
  dbg!(&sandyq);
}

fn parse(session: &Session) -> Result<Sandyq> {
  let mut parser = match &session.input_file {
    Input::File(path) => new_parser_from_file(&session.psess, path),
  }?;

  let sandyq = parser.parse_sandyq().map_err(|diag| {
    dbg!(&diag);
    diag.emit()
  });
  sandyq
}
