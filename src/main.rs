use read_input::prelude::*;
use std::io::Write;
use std::io::{self, stdin};

fn main() {
    loop {
        header();

        print!("\n\tEnter your decimal number: ");
        io::stdout().flush().unwrap();
        let dec = input().get();

        if dec > 0 {
            let binary_array = calc_binary(dec);
            println!("\n\tYour calculated binary number: {:?}", binary_array)
        } else {
            println!("\n\tCant calculate an number decimal less then 1");
        }
        if !new_run("\n\tDo you want to make a new Calculation?(y/n): ") {
            break;
        }
    }
}

fn header() {
    println!("\n\tBinary Calculator");
    println!("\t================");
}

fn calc_binary(mut dec: i32) -> [i32; 9] {
    let mut bin_array: [i32; 9] = [0; 9];

    for i in (0..9).rev() {
        bin_array[i] = dec % 2;
        dec /= 2;
    }
    bin_array
}

fn new_run(text: &str) -> bool {
    print!("\n\t{}", text);
    io::stdout().flush().unwrap();
    if user_input().map(|inp| inp.to_lowercase()) == Some("y".to_owned()) {
        true
    } else {
        false
    }
}

fn user_input() -> Option<String> {
    stdin().lines().next()?.ok()
}
