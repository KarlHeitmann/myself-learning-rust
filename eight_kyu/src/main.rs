fn name_shuffle(s: &str) -> String {
//fn name_shuffle(s: &str) -> &str {
  /*
  // https://www.codewars.com/kata/559ac78160f0be07c200005a/train/rust
  println!("{}", s);
  //let (name: &str, last: &str) = s.split_once(' ').unwrap();
  let (name, last) = s.split_once(' ').unwrap();
  //return concat!(last, name);
  return last + name;
  */
  let (name, last) = s.split_once(' ').unwrap();
  //return concat!(last, name);
  return last.to_string() + " " + name;
}

fn main() {
  let t1 = name_shuffle("john McClane");
  let t2 = name_shuffle("Mary jeggins");
  let t3 = name_shuffle("tom jerry");
  println!("{}", t1);
  println!("{}", t2);
  println!("{}", t3);
}

