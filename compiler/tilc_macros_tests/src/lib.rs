#[cfg(test)]
mod tests {
  use tilc_macros::uidx;

  #[test]
  fn uidx() {
    uidx! {
      #[max = 2]
      pub struct Idx {}
    }
    let mut idx = Idx::from_u16(1);
    println!("Before: {:?}", idx);

    idx = idx + 1;
    println!("After: {:?}", idx);
  }
}
