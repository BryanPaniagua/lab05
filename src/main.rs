use std::io;

fn main() {
    println!("Enter any number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Not a number");
    println!("The number is: {}", input);

    let total_count = count_divisors(input);
   
    println!("Total number of divisors: {}", total_count);
}

fn count_divisors(n: i32) -> i32 {
    let mut count = 0;
    println!("The divisors of {} are:", n);

    for i in 1..=n {

        if n % i == 0 {
            print!("{} ", i);
            count += 1;
        }
    }
    println!();

    count
}
