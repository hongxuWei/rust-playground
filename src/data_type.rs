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
        println!("Parsed u8 value: {}", parsed_value);
        // 当 cargo run data-type -- --data-type-integer-overflow 255 时 会 panic
        // 当执行 cargo build --release 后，这里会打印 0
        println!("Parsed u8 + 1 is: {}", parsed_value + 1);
    } else {
        println!("Failed to parse value as u8");
    }
  } else {
      println!("Value is None");
  }
}

