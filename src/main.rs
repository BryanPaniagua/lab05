use std::io;
fn main() {
 

	println!("Enter any number");
	let mut input = String::new();
	io::stdin().read_line(&mut input);
	let number: i32 = input.trim().parse().expect("Please type a number!");
	if is_even(number) { 
		println!("Even");
	}

	println!("Enter numbers");
	let mut input = String::new();
	io::stdin().read_line(&mut input);
	let number: i32 = input.trim().parse().expect("Please type a number!");
	println!("Sum of digits: {}", digit_sum(number));
}

fn is_even(n: i32) -> bool {
	n % 2 == 0

}

fn digit_sum(mut number: i32) -> i32 {	
	let mut sum = 0;
	while number > 0 {
		sum += number % 10;
		number /= 10;
	}
        sum
}
