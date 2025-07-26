use std::sync::OnceLock;

use tilc_ast::Sandyq;
use tilc_data_structure::Holder;
use tilc_expand::ExpansiontCtxt;
use tilc_middle::{Arena, QueryCaches, QuerySystem, QuerySystemFns, TyCtxt};
use tilc_parse::new_parser_from_file;
use tilc_resolver::Resolver;
use tilc_session::{Input, Session};
use tilc_span::{SandyqId, sym};

use crate::{DEFAULT_QUERY_PROVIDERS, Result, compiler::Compiler};


pub(crate) fn resolver_for_lowering<'ctxt>(
  tcx: TyCtxt<'ctxt>,
  _: (),
) -> &'ctxt Holder<Sandyq> {
  let mut sandyq = tcx.crate_for_resolving(()).steal();

  let arenas = Resolver::arenas();
  let mut resolver = Resolver::new(tcx, &arenas, sandyq.span);

  let mut expansion_ctxt = ExpansiontCtxt::new(tcx.session, &mut resolver);
  sandyq = expansion_ctxt
    .monotonic_expander(true)
    .expand_sandyq(sandyq);
  resolver.resolve_sandyq(&sandyq);
  dbg!(&resolver);

  return tcx.arena.alloc(Holder::new(sandyq));
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

  let providers = *DEFAULT_QUERY_PROVIDERS;


  let gcx_cell = OnceLock::new();
  let arena = Arena::default();
  let sandyq_id = SandyqId::new(sym::tilc_out, false);

  return TyCtxt::create_global_ctxt(
    &gcx_cell,
    session,
    &arena,
    QuerySystem {
      fns: QuerySystemFns {
        local_providers: providers.queries,
      },
      caches: QueryCaches::default(),
    },
    sandyq_id,
    |tcx| {
      let feed = tcx.unit_query_feed();

      feed.crate_for_resolving(tcx.arena.alloc(Holder::new(sandyq)));

      return f(tcx);
    },
  );
}
