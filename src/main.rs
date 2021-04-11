mod company;
mod strcode;
mod calculations;

use std::io;
use rand::Rng;
use company::Company;
use company::Command;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let numbers: Vec<i32> = vec![5,2,4,3,5,1,1,1];
    println!("mean - {}", calculations::mean(&numbers));
    println!("median - {}", calculations::median(&numbers));
    println!("mode - {}", calculations::mode(&numbers));
    println!("pig latin - {}", strcode::as_pig_latin("   this   make me say oi-vey ok"));

    let mut company = Company::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("bad input");
        let command = company::parse_command(&input);
        if command == Command::Exit {
            break;
        }
        company.execute(command);
    }
}
