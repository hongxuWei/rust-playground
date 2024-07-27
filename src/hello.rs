pub fn run_hello() {
  println!("Hello, world!");

  println!("{} days", 31);
  // {n} 参数顺序
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // named argument
  println!(
    "{subject} {verb} {object}",
    object = "the lazy dog",
    subject = "the quick brown fox",
    verb = "jumps over",
  );

  // :b 二进制输出, fomatted after ":"
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

  // :>n$ pad spacing to fill the [number] length to width
  println!("{number:>width$}", number = 1, width = 2);

  // :>xn$ pad x to fill the [number] length to width
  println!("{number:0>width$}", number = 1, width = 2);

  // println!("My name is {0}, {1} {0}", "Bond");
  // FIXME ^ Add the missing argument: "James"

  #[allow(dead_code)]
  struct Structure(i32);

  let number: f64 = 1.0;
  let width: usize = 2;
  println!("{number:>width$}");
}
