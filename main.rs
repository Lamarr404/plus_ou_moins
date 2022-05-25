use std::io::stdin;
use rand::Rng;

fn main() {

	
	
	let number = rand::thread_rng().gen_range(1..101);

	
	println!("le nombre random est: {}", number);
    
}
