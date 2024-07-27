fn sum(x: i32, y: i32) -> i32 {
  let result = x + y;
  println!("x is {}, y is {}, x + y is = {}", x, y, result);
  return result;
}

pub fn run_fn() {
  sum(1, 2);

  let y = {
    let x = 3;
    x + 1
  };
  println!("let y = {{\n  let x = 3;\n  let x + 1\n}};\nthen you got y is {}", y);
}
