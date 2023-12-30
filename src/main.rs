mod hello;
mod variable;
mod data_type;
mod utils;
mod function;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let run_mode = args.get(1);

  if let Some(mode) = run_mode {
    match mode.as_str() {
      "hello" => hello::run_hello(),
      "variable" => variable::run_variable(),
      "data-type" => data_type::run_data_type(),
      "fn" => function::run_fn(),
      _ => {
        println!("{} 没有该命令，", mode);
      }
    }
  } else {
    println!("没有找到任何可用的运行模式。 请执行：cargo run [mode]");
  }
}