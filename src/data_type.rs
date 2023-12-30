use crate::utils::get_command_line_argument;

pub fn run_data_type() {
  let u8dec = 100;
  let u8hex = 0x64;
  let u8octal = 0o144;
  let u8bin = 0b0110_0100;
  let u8byte = b'd';

  println!("整型 {u8dec} {u8hex} {u8octal} {u8bin} {u8byte}");

  let arg_of_data_type_integer_overflow = get_command_line_argument("--data-type-integer-overflow");

  if let Some(string_value) = arg_of_data_type_integer_overflow {
    if let Ok(parsed_value) = string_value.parse::<u8>() {
        println!("输入的值转为 u8 类型后是: {}", parsed_value);
        // 当 cargo run data-type -- --data-type-integer-overflow 255 时 会 panic
        // 当执行 cargo build --release 后，这里会打印 0
        println!("输入的值转为 u8 类型并且 + 1 后是: {}", parsed_value + 1);
    } else {
        println!("Failed to parse value as u8");
    }
  } else {
      println!("执行 cargo run data-type -- --data-type-integer-overflow 255 会抛错\n执行 cargo build --release && ./target/release/rust-demo data-type --data-type-integer-overflow 255 会整型溢出");
  }

  // 浮点数
  let float_x: f64 = 2.0;
  let float_y: f32 = 2.0;
  println!("float_x: f64 {}, float_y: f32 {}",float_x, float_y);

  // addition
  let sum = 5 + 10;
  println!("5 + 10 = {}", sum);


  // subtraction
  let difference = 95.5 - 4.3;
  println!("95.5 - 4.3 = {}", difference);

  // multiplication
  let product = 4 * 30;
  println!("4 * 30 = {}", product);


  // division
  let quotient = 56.7 / 32.2;
  println!("56.7 / 32.2 = {}", quotient);

  let truncated = -5 / 3; // 结果为 -1
  println!("-5 / 3 = {} it's a int", truncated);

  // remainder
  let remainder = 43 % 5;
  println!("43 % 5 = {}", remainder);

  let bool_x = true;
  println!("bool_x is {}", bool_x);

  // 复合类型
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // 解构取值
  let (x, y, z) = tup;
  println!("x, y, z in tup is {} {} {}", x, y, z);

  // 索引取值
  println!("tup.0, tup.1, tup.2 in tups is {} {} {}", tup.0, tup.1, tup.2);

  let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
  println!("months[0] is {}", months[0]);
  let arr_i32_a: [i32; 5] = [1, 2, 3, 4, 5];
  println!("arr_i32_a[0] is {}", arr_i32_a[0]);
  let arr_a = [3; 5];
  println!("arr_a[4] is {}", arr_a[4]);
}

