use std::{path::PathBuf, rc::Rc};


use crate::{compiler::Compiler, interface};
use crate::{util, Computation};

use tilc_ast::Sandyq;
use tilc_backend::Backend;
use tilc_backend_llvm::LLVMBackend;
use tilc_errors::Diag;
use tilc_parse::new_parser_from_file;
use tilc_parser::Parser;
use tilc_session::{Config, Input, Options, ParseSession, Session, Target, IO};
use tilc_span::{ActualFileLoader, FileLoader, SessionGlobals, SourceMap};


pub struct Runner<'a> {
  args: &'a [String],
}
impl<'a> Runner<'a> {
  pub fn new(args: &'a [String]) -> Self {
    return Self { args };
  }

  pub fn run(&self) -> interface::Result<()> {
    let input: Input =
      Input::File(PathBuf::new().join(self.args.get(0).unwrap_or_else(|| {
        panic!("Empty input path");
      })));

    let config: Config = Config {
      options: Options {},
      input,

      raw_args: self.args.to_vec(),
    };


    let sys_root: PathBuf = util::get_sys_root_path();
    let file_loader: Box<dyn FileLoader> = Box::new(ActualFileLoader);

    // FIXME: Remove hardcoded backend choice
    //        Add feature to choose backend from config options
    //        but anyway there are a lot of time until implementing second backend
    let backend: Box<dyn Backend> = Box::new(LLVMBackend::new());


    let source_map: Rc<SourceMap> =
      tilc_span::with_session_globals(|session_globals: &SessionGlobals| {
        return session_globals.source_map.clone().unwrap();
      });
    let parse_session: ParseSession = ParseSession::new(source_map);

    // FIXME: Terrible but fine for now
    let session: Session = Session {
      target: Target {
        architecture: "x86_64".to_string(),
        triplet: "doesnt-matter-now".to_string(),
      },
      opts: config.options,

      parse_session,
      sys_root,
      io: IO {
        input: config.input,
        output_dir: PathBuf::new().join("build"),
        output_file: PathBuf::new().join("out"),
      },
    };
    let compiler: Compiler = Compiler::new(session, backend);

    compiler.enter(|computation: &Computation<'_>| {
      computation.parse()?;
      println!("{:#?}", computation.parse()?);

      return Ok(());
    })?;

    // let sandyq: Sandyq = parse(&compiler.session)?;
    // println!("{:#?}", sandyq);

    todo!()
  }
}


pub fn parse(session: &Session) -> interface::Result<Sandyq> {
  let mut parser: Parser<'_> = match &session.io.input {
    Input::File(path) => new_parser_from_file(&session.parse_session, path),
    Input::Str(_) => todo!(),
  }?;


  let sandyq: Sandyq =
    parser.parse_sandyq().map_err(|diag: Diag| diag.emit())?;

  return Ok(sandyq);
}
