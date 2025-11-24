use std::{
  path::PathBuf,
  sync::{Arc, OnceLock},
};

use tilc_ast::Sandyq;
use tilc_data_structures::Holder;
use tilc_error::FatalError;
use tilc_middle::{
  Arena, DEFAULT_QUERY_PROVIDERS, QueryCaches, QueryFns, QuerySystem, TyCtxt, queries::Providers,
};
use tilc_parse::new_parser_from_file;
use tilc_session::{Input, ParseSession, Session};
use tilc_span::with_session_globals;

use crate::Result;

pub fn runner(args: &[String]) -> Result<()> {
  let input_file = Input::File(match args.get(0) {
    Some(path) => PathBuf::from(path),
    None => FatalError.raise(),
  });
  let source_map = with_session_globals(|session_globals| session_globals.source_map());
  let psess = ParseSession::new(source_map);
  let session = Session { input_file, psess };

  let sandyq = parse(&session)?;
  dbg!(&sandyq);

  create_and_enter_global_ctxt(&session, sandyq, |tcx| {
    let _ = tcx.resolver_for_lowering_raw(());
  });

  return Ok(());
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

fn create_and_enter_global_ctxt<F, R>(session: &Session, sandyq: Sandyq, f: F) -> R
where
  F: for<'ctxt> FnOnce(TyCtxt<'ctxt>) -> R, {
  let gcx_cell = OnceLock::new();
  let arena = Arena::new();

  let providers = *DEFAULT_QUERY_PROVIDERS;

  let query_system = QuerySystem {
    fns: QueryFns {
      local_providers: providers,
    },
    caches: Default::default(),
  };

  TyCtxt::create_global_ctxt(&gcx_cell, session, &arena, query_system, |tcx| {
    let feed = tcx.feed(());
    feed.sandyq_for_resolver(tcx.arena.alloc(Holder::new(sandyq)));

    f(tcx)
  })
}
