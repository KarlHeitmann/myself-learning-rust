// https://www.codewars.com/kata/544aed4c4a30184e960010f4/solutions
fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    /*
	// panic!("todo: implement divisors")
    let mut output = Vec::new();
    for n in 2..integer {
        if (integer % n) == 0 {
            output.push(n);
        }
    }
    */
    let divs = (2..integer)
        .filter(|k| integer % k == 0)
      .collect::<Vec<u32>>();
      
    if divs.len() > 0 {
        Ok(divs)
    } else {
        Err(format!("{} is prime", integer))
    }
}

pub fn run() {
    println!("DIVISORS RUN");
    println!("{:?}", divisors(15)); //, Ok(vec![3,5]));
    println!("{:?}", divisors(12)); //, Ok(vec![2,3,4,6]));
    println!("{:?}", divisors(13)); //, Err("13 is prime".to_string()));
}
