use std::io::{self, stdout};

fn main() {
    let mut x: String = String::new();
    let mut y: String = String::new();
    let mut op: String = String::new();
    let res: i32;

    println!("Enter the first number: ");
    io::stdin().read_line(&mut x).expect("Invalid Input");

    let x: i32 = match x.trim().parse() {
        Ok(num) => num, 
        Err(_)=> {
            println!("Invalid first number!");
            return;
        }
    };

    println!("Enter the second number: ");
    io::stdin().read_line(&mut y).expect("Invalid Input");

    let y: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid second number!");
            return;
        }
    };

}
