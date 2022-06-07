// https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/solutions/rust
fn even_or_odd(n: i32) -> &'static str {
    match n % 2 {
        0 => "Even",
        _ => "Odd"
    }
}
pub fn run () {
    println!("{}", even_or_odd(0));
    println!("{}", even_or_odd(2));
    println!("{}", even_or_odd(1));
    println!("{}", even_or_odd(7));
    println!("{}", even_or_odd(-1));
}
