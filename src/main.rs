extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
	println!("Угадай число!");
	
	let secret_number = rand::thread_rng().gen_range(1,101);
	
	//println!("Секретное число: {}", secret_number);
	
	loop{
		println!("Введи число");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Не шмогла ");

		let guess: u32 = match guess.trim().parse() {
			Ok(num)=>num,
			Err(_)=>continue,
		};

		println!("Угадал: {}",guess);

		match guess.cmp(&secret_number) {
			Ordering::Less    => println!("маленький!"),
			Ordering::Greater => println!("большой!"),
			Ordering::Equal   => {
					println!("победа!");
				 	break;
				},
		}
	}
}