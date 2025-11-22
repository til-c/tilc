use tilc_interface::{catch_if_error, runner};

fn main() {
  let args: Vec<String> = std::env::args().map(|arg| arg).collect();

  let exit_code = match catch_if_error(move || {
    runner(&args[1..]);
  }) {
    Ok(_) => 0,
    Err(_) => 1,
  };
  std::process::exit(exit_code);
}
