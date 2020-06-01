use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Guess the number!");
    println!("Enter the guees!");
    loop {
		let mut input = String::new();
		io::stdin().
			read_line(&mut input).
			expect("failed to readline!");
		let guess :u32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		println!("your guess is {}", input);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You win!");
                break;

            }        
        }
    }
}
