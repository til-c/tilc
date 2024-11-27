use tilc_backend::Backend;


pub struct LLVMBackend;
impl LLVMBackend {
  pub fn new() -> Self {
    return Self;
  }
}
impl Backend for LLVMBackend {}
