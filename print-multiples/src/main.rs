use std::io;
use std::vec;


fn main() {
    println!("How many multiples would you like to calculate?: ");
    let mut size_input = String::new();
    io::stdin().read_line(&mut size_input).expect("Failed to read line");

    let size: usize = size_input.trim().parse().expect("Please type a number");

    println!("What number would you like to multiply?: ");

    let mut num_input = String::new();
    io::stdin().read_line(&mut num_input).expect("Failed to read line");

    let num: usize = num_input.trim().parse().expect("Please type a number");
    let mut array = vec![];();

    for i in 0..size {
        let element: usize = num * (i + 1);
        array.push(element);
    }
    println!("The first {} multiples of {} are {:?}", size, num, array);
}

