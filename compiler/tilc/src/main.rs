use tilc_interface::{catch_if_error, Compiler};


fn main() -> ! {
  // let args = {
  //   let mut vec: Vec<String> = Vec::new();
  //   for (i, arg) in std::env::args().enumerate() {
  //     vec.push(arg);
  //   }
  //   vec
  // };
  let args: Vec<String> =
    std::env::args().enumerate().map(|(_, arg)| arg).collect();


  let exit_code: i32 = match catch_if_error(|| {
    return Compiler::new(&args).run();
  }) {
    Ok(_) => 0,
    Err(_) => 1,
  };
  std::process::exit(exit_code)
}
