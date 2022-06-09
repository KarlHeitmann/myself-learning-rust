pub fn run() {
    println!("{}", square_sum(vec![1, 2])); //, 5);
    println!("{}", square_sum(vec![-1, -2])); //, 5);
    println!("{}", square_sum(vec![5, 3, 4])); //, 50);
    println!("{}", square_sum_fold(vec![5, 3, 4])); //, 50);
    println!("{}", square_sum_best_practice(vec![5, 3, 4])); //, 50);
    println!("{}", square_sum(vec![])); //, 0);
}

fn square_sum(vec: Vec<i32>) -> i32 {
    let mut result = 0;
    for n in vec {
        result = result + (n * n);
    }
    result
}

fn square_sum_fold(v: Vec<i32>) -> i32 {
    let sum: i32 = v
        .iter()
        .fold(0i32, |mut sum, val| {sum = sum + (val * val); sum});
    return sum;
}

fn square_sum_best_practice(vec: Vec<i32>) -> i32 {
    // vec.iter().map(|v| v.pow(2)).sum()
    // vec.iter().map(|s| s * s).sum()
    // 
    vec.iter().fold(0, |t,i| t + i*i)

}
