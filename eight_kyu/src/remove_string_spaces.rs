
fn no_space(s: &str) -> String {  
    /*
    let char_vec: Vec<char> = x.chars().collect();
    let mut output = Vec::new();
    for c in char_vec {
        if c != ' ' {
            output.push(c);
        }
    }
    output.into_iter().collect()
    */
    // s.chars().into_iter().collect()
    // array_iter.filter(|x| { let _: () = x; x == 2 });
    s.chars().filter(|c| *c != ' '  ).into_iter().collect()
}

#[test]
fn returns_expected() {
  assert_eq!("8j8mBliB8gimjB8B8jlB".to_string(), no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string()));
  assert_eq!("88Bifk8hB8BB8BBBB888chl8BhBfd", no_space("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd".to_string()));
  assert_eq!("8aaaaaddddr", no_space("8aaaaa dddd r     ".to_string()));
  assert_eq!("jfBmgklf8hg88lbe8", no_space("jfBm  gk lf8hg  88lbe8 ".to_string()));
  assert_eq!("8jaam", no_space("8j aam".to_string()));        
}

pub fn run() {
    //println!("Remove string spaces");
    returns_expected();
}

