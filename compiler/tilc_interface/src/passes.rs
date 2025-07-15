use std::sync::{Arc, OnceLock};

use tilc_ast::Sandyq;
use tilc_middle::TyCtxt;
use tilc_parse::new_parser_from_file;
use tilc_session::{Input, Session};
use tilc_span::{SandyqId, sym};

use crate::{Result, compiler::Compiler};


pub(crate) fn resolver_for_lowering<'ctxt>(
  tcx: TyCtxt<'ctxt>,
  _: (),
) -> Arc<Sandyq> {
  todo!("123");
}

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

pub(crate) fn create_and_enter_global_ctxt<
  T,
  F: for<'ctxt> FnOnce(TyCtxt<'ctxt>) -> T,
>(
  compiler: &Compiler,
  sandyq: Sandyq,
  f: F,
) -> T {
  let session = &compiler.session;

  let gcx_cell = OnceLock::new();
  let sandyq_id = SandyqId::new(sym::tilc_out, false);
  return TyCtxt::create_global_ctxt(&gcx_cell, session, sandyq_id, f);
}
