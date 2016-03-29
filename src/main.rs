use std::io;

fn main(){
	println!("Угадай число!");
	
	println!("Введи число");
	
	let mut guess = String::new();
	
	io::stdin().read_line(&mut guess)
		.expect("Не шмогла ");
	
	println!("Угадал: {}",guess);
}