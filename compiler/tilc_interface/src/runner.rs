use std::path::PathBuf;

use tilc_error::FatalError;
use tilc_session::{CompilerIO, Input, ParseSession, Session};
use tilc_span::with_session_globals;

use crate::{Result, compiler::Compiler, create_and_enter_global_ctxt};


pub struct Runner<'a> {
  args: &'a [String],
}
impl<'a> Runner<'a> {
  pub fn new(args: &'a [String]) -> Self {
    return Self { args };
  }


  /// Main entry point
  pub fn run(&self) -> Result<()> {
    let input = Input::File(match self.args.get(0) {
      Some(path) => PathBuf::from(path),

      None => FatalError::raise(),
    });
    let source_map = with_session_globals(|session_globals| {
      return session_globals.source_map.clone().unwrap();
    });
    let parse_session = ParseSession::new(source_map);

    let session = Session {
      io: CompilerIO {
        input,
        output_dir: PathBuf::from("build"),
        output_file: PathBuf::from("out"),
      },
      parse_session,
    };
    let compiler = Compiler { session };


    let sandyq = crate::parse(&compiler.session)?;
    // dbg!(&sandyq);


    create_and_enter_global_ctxt(&compiler, sandyq, |tcx| {
      tcx.resolver_for_lowering(());
    });


    return Ok(());
  }
}
