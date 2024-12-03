use std::io;
use std::vec;

fn calc_multiples(num: u32, amount_of_multiples: u32) -> Vec<u32> {

    let mut array = vec![];();

    for i in 0..amount_of_multiples{
        let element: u32 = num * (i + 1);
        array.push(element);
    }
    array
}

fn main() {
    println!("How many multiples would you like to calculate?: ");
    let mut size_input = String::new();
    io::stdin().read_line(&mut size_input).expect("Failed to read line");

    let size: u32 = size_input.trim().parse().expect("Please type a number");

    println!("What number would you like to multiply?: ");

    let mut num_input = String::new();
    io::stdin().read_line(&mut num_input).expect("Failed to read line");

    let num: u32 = num_input.trim().parse().expect("Please type a number");

    println!("The first {} multiples of {} are {:?}", size, num, calc_multiples(num, size));
}

