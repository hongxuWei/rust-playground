use std::env;

pub fn get_command_line_argument(key: &str) -> Option<String> {
  let args: Vec<String> = env::args().collect();

  for (index, arg) in args.iter().enumerate() {
      if arg == key {
          if let Some(value) = args.get(index + 1) {
              return Some(value.clone());
          }
      }
  }

  None
}
