use tilc_interface::{Runner, catch_if_error};


fn main() -> ! {
  // TODO: Configuration options
  // Note: For now first arg is must be a path to a file to compile
  let args: Vec<String> = std::env::args().map(|arg| arg).collect();


  let exit_code = match catch_if_error(|| {
    return Runner::new(&args[1..]).run();
  }) {
    Ok(_) => 0,
    Err(_) => 1,
  };
  std::process::exit(exit_code);
}
