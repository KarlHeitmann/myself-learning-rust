// use nameshuffler;
use std::env;

mod kyu6;
mod kyu7;
mod kyu8;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() == 1 {
        kyu8::square_n_sum::run();
    } else {
        if args[1] == "name_shuffler" {
            kyu8::nameshuffler::run();
        } else if args[1] == "replace_all_dots" {
            kyu8::fixme_replace_all_dots::run();
        } else if args[1] == "remove_string_spaces" {
            kyu8::remove_string_spaces::run();
        } else if args[1] == "plurals" {
            kyu8::plurals::run();
        } else if args[1] == "bouncing_ball" {
            kyu6::bouncing_ball::run();
        } else if args[1] == "square_n_sum" {
            kyu8::square_n_sum::run();
        } else if args[1] == "divisors" {
            kyu7::divisors::run();
        } else if args[1] == "even_or_odd" {
            kyu8::even_or_odd::run();
        }
    }
}

