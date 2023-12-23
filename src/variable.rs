pub fn run_variable() {
  // 变量 variable
  // let x = 5;  // this line will get a error
  let mut x = 5;
  println!("The value of x is： {x}");
  x = 6;
  println!("The value of x is： {x}");

  // 常量 constants
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("constants THREE_HOURS_IN_SECONDS value is: {THREE_HOURS_IN_SECONDS}");

  // shadowing
  let y = 5;
  println!("The value of y is: {y}");
  let y = y + 1;
  println!("The value of y is: {y}");
  let y = y + 100;
  println!("The value of y is: {y}");

  let z = 5;

  {
      let z = z * 2;
      println!("The value of z in the inner scope is: {z}");
  }

  println!("The value of z is: {z}");
}

