// use nameshuffler;
use std::env;

/*
mod nameshuffler;
mod fixme_replace_all_dots;
mod remove_string_spaces;
mod even_or_odd;
mod plurals;
*/
mod divisors;
mod kyu8;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() == 1 {
        divisors::run();
    } else {
        if args[1] == "name_shuffler" {
            kyu8::nameshuffler::run();
        } else if args[1] == "replace_all_dots" {
            kyu8::fixme_replace_all_dots::run();
        } else if args[1] == "remove_string_spaces" {
            kyu8::remove_string_spaces::run();
        } else if args[1] == "plurals" {
            kyu8::plurals::run();
        } else if args[1] == "divisors" {
            divisors::run();
        } else if args[1] == "even_or_odd" {
            kyu8::even_or_odd::run();
        }
    }
}

