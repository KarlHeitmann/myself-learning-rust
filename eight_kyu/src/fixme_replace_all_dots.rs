use regex::Regex;

// https://www.codewars.com/kata/596c6eb85b0f515834000049/train/rust
pub fn run() {
  println!("{}", replace_dots(""));
  println!("{}", replace_dots("no dots"));
  println!("{}", replace_dots("one.two.three"));
  println!("{}", replace_dots("........"));

  println!("=========================");
  println!("=========================");
  println!("=========================");

  println!("{}", replace_dots_fix(""));
  println!("{}", replace_dots_fix("no dots"));
  println!("{}", replace_dots_fix("one.two.three"));
  println!("{}", replace_dots_fix("........"));

  println!("{}", replace_dots_match(""));
  println!("{}", replace_dots_match("no dots"));
  println!("{}", replace_dots_match("one.two.three"));
  println!("{}", replace_dots_match("........"));
  /*
  assert_eq!(replace_dots(""), "");
  assert_eq!(replace_dots("no dots"), "no dots");
  assert_eq!(replace_dots("one.two.three"), "one-two-three");
  assert_eq!(replace_dots("........"), "--------");
  */
}

fn replace_dots(s: &str) -> String {
  let char_vec: Vec<char> = s.chars().collect();
  /*
  let mut output = String.new();
  for c in char_vec {
    println!("{}", c);
    let n = if c == '.' { '-' } else { c };
    output.push_str(n);
  }
  // "Welcome fixme replace all dots".to_string()
  // */
  let mut output = Vec::new();
  for c in char_vec {
    //println!("{}", c);
    //let n = if c == '.' { '-' } else { c };
    let n = if c == '.' { '-' } else { c };
    output.push(n);
  }
  //output
  //println!("{}", s);
  /*
  let st: String = outpuvt.into_iter().collect();
  st
  */
  output.into_iter().collect()
}

fn replace_dots_fix(s: &str) -> String {
  Regex::new(r"\.").unwrap().replace_all(s, "-").to_string()
}

fn replace_dots_match(s: &str) -> String {
    s.chars().map(|c| match c { '.' => '-', _ => c }).collect()
}

