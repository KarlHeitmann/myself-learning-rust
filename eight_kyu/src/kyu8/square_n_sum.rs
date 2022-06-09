pub fn run() {
    println!("{}", square_sum(vec![1, 2])); //, 5);
    println!("{}", square_sum(vec![-1, -2])); //, 5);
    println!("{}", square_sum(vec![5, 3, 4])); //, 50);
    println!("{}", square_sum(vec![])); //, 0);
}

pub fn square_sum(vec: Vec<i32>) -> i32 {
    let mut result = 0;
    for n in vec {
        result = result + (n * n);
    }
    result
}
