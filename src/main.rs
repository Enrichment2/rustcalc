use terminal_fonts::{to_block_string};
use std::io::{self, Write};
use clearscreen;

fn printMenu() {
		println!("{}", to_block_string("RUST CALC"));
		println!("__________________________________________________________________________");
		println!("Please type the number of the option you want to choose, then press enter!");
		println!("1) Addition");
		println!("2) Subtraction");
		println!("3) Division");
		println!("4) Multiplication");
		println!("0) Exit");
}

fn getNumber() -> f64 {
	loop {
		io::stdout().flush().unwrap();
		let mut userInput = String::new();
		io::stdin().read_line(&mut userInput).expect("Failed, try again!");
		match userInput.trim().parse::<f64>(){
			Ok(number) => {
				return number;
			},
			Err(_) => {
				println!("Nope try again");
				print!("> ");
				continue;
			}
		}
	}
} 
fn main() {
	let mut error_message = String::new();
	let mut choice = 0;
	loop {
		clearscreen::clear().unwrap();
		printMenu(); // Prints the main menu
		if !error_message.is_empty(){
			println!("{}", error_message);
			error_message.clear();
		}
		print!("> "); // Prints the input cursor
		let menuInput = getNumber();
		
		if menuInput == 0.0 {
			break;
		}
		if menuInput >= 1.0 && menuInput <= 4.0 {
			choice = menuInput as i32;
		} else {
			error_message = String::from("Choose a number 0 to 4 you absolute fool");
		}
	
	
	if choice == 1 {
		println!("You chose addition!");
		println!("Input the first number of the addition equation");
		print!("> ");
		let firstDigi = getNumber();
		println!("Great! Now input the second number of your addition equation");
		print!("> ");
		let secondDigi = getNumber();
		let answer = firstDigi + secondDigi;
		println!("{}+{}={}", firstDigi, secondDigi, answer);
	}
	
	if choice == 2 {
		println!("You chose subtraction!");
		println!("Input the first number of the subtraction equation");
		print!("> ");
		let firstDigi = getNumber();
		println!("Great! Now input the second number of your subtraction equation");
		print!("> ");
		let secondDigi = getNumber();
		let answer = firstDigi - secondDigi;
		println!("{}-{}={}", firstDigi, secondDigi, answer);
	}
	
	if choice == 3 {
		println!("You chose division!");
		println!("Input the first number of the division equation");
		print!("> ");
		let firstDigi = getNumber();
		println!("Great! Now input the second number of your division equation");
		print!("> ");
		let secondDigi = getNumber();
		if secondDigi == 0.0 {
			println!("Can't divide by 0 fool");
		} else {
			let answer = firstDigi / secondDigi;
			println!("{}/{}={}", firstDigi, secondDigi, answer);
		}
	}
	
	if choice == 4 {
		println!("You chose multiplication!");
		println!("Input the first number of the multiplication equation");
		print!("> ");
		let firstDigi = getNumber();
		println!("Great! Now input the second number of your multiplication equation");
		print!("> ");
		let secondDigi = getNumber();
		let answer = firstDigi * secondDigi;
		println!("{}x{}={}", firstDigi, secondDigi, answer);
	}
	println!("1) Back");
	println!("0) Exit");
	print!("> "); // Prints the input cursor
		let menuInput = getNumber();
		
		if menuInput == 0.0 {
			break;
		}
		if menuInput == 1.0 {
			continue;
		} else {
			error_message = String::from("Choose a number 0 to 4 you absolute fool");
		}
	}
}
