type Result<V, E> {
  Err(E),
  Ok(V)
}

fn error(num: i32) -> Result<(), usize> {
  if num < 0 {
      return Err((num * -1) as usize);
  }

  return Ok(());
}

fn main() -> Result<(), usize> {
  let res = error(5);

  if res.is_ok() {
      //...
  }

  match res {
      Err(e) => // ...
      Ok(v) => // ...
  }

  let x = res.unwrap_or(());
  let x = res.expect("THIS BETTER EXIST");
  let x = res.unwrap(); // BAD

  let x = res?;

  return Ok(());
}