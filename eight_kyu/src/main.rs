// use nameshuffler;
use std::env;

mod nameshuffler;
mod fixme_replace_all_dots;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  if args.len() == 1 {
    fixme_replace_all_dots::run();
  } else {
    if args[1] == "name_shuffler" {
      nameshuffler::run();
    } else if args[1] == "replace_all_dots" {
      fixme_replace_all_dots::run();
    }
  }
}

