mod hello;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let run_mod = &args[1];

  let _hello: String = String::from("hello");

  match run_mod {
    _hello => hello::hello(),
  }
}