use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter a list of numbers, separated by commas:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split(",")
        .map(|num| num.trim().parse().unwrap())
        .collect();

    let sum: i32 = numbers.iter().sum();
    let average = sum as f64 / numbers.len() as f64;

    println!("Sum: {}", sum);
    println!("Average: {}", average);
}
