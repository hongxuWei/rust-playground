pub fn run_control_flow() {
  // if
  let number = 3;
  if number < 5 {
    println!("number < 5");
  } else {
    println!("number > 5");
  }
  
  // 下面将会抛错 expected `bool`, found integer
  // if number {
  //   println!("number is bool");
  // }

  // 在 let 中用 if
  let condition = true;
  let number = if condition { 5 } else { 6 };
  println!("number is {number}");

  // loop
  // 下面是个死循环
  // loop {
  //   println!("loop again!");
  // }

  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };
  println!("result is {result}");
}