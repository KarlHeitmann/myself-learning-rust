// https://www.codewars.com/kata/52ceafd1f235ce81aa00073a

pub fn run() {
    println!("{}", plural(0.0)); // true
    println!("{}", plural(0.5)); // true
    println!("{}", plural(1.0)); // falseº
    println!("{}", plural(100.0)); // true
}

fn plural (n: f64) -> bool {
    // your code here
    /*
    match n {
        1.0 => false,
        _ => true,
    }
    */
    n != 1.0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural() {
    }
}

