use std::io;
fn main() {
 
fn is_even(n: i32) -> bool {
	n % 2 == 0

}
	println!("Enter any number");
	let mut input = String::new();
	io::stdin().read_line(&mut input);
	let number: i32 = input.trim().parse().expect("Please type a number!");
	if number % 2 == 0 {
		println!("{number} yes");
	}
	else {
		println!("Not Even");
		
	}


	
}
