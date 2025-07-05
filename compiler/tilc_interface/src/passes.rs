use tilc_ast::Sandyq;
use tilc_parse::new_parser_from_file;
use tilc_session::{Input, Session};

use crate::Result;

pub(crate) fn parse(session: &Session) -> Result<Sandyq> {
  let mut parser = match &session.io.input {
    Input::File(path) => new_parser_from_file(&session.parse_session, path),
    _ => todo!(),
  }?;
  let sandyq = parser.parse_sandyq().map_err(|diag| {
    dbg!(&diag);
    return diag.emit();
  })?;


  return Ok(sandyq);
}
