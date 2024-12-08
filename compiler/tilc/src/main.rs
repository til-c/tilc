use tilc_interface::{catch_if_error, Runner};


fn main() -> ! {
  // TODO: Configuration options
  // NOTE: For now compiler assumes that first arg is always path to a file
  let args: Vec<String> = std::env::args()
    .enumerate()
    .map(|(_, arg): (_, String)| arg)
    .collect();


  let exit_code: i32 = match catch_if_error(|| {
    return Runner::new(&args[1..]).run();
  }) {
    Ok(_) => 0,
    Err(_) => 1,
  };
  std::process::exit(exit_code)
}
