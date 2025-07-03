use crate::Result;


pub struct Runner<'a> {
  args: &'a [String],
}
impl<'a> Runner<'a> {
  pub fn new(args: &'a [String]) -> Self {
    return Self { args };
  }


  /// Main entry point
  pub fn run(&self) -> Result<()> {
    return Ok(());
  }
}
