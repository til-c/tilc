use tilc_interface::catch_if_error;


fn main() -> ! {
  let exit_code = match catch_if_error(|| {
    // TODO: Run compiler here
  }) {
    Ok(_) => 0,
    Err(_) => 1,
  };
  std::process::exit(exit_code);
}
