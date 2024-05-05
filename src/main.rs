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

    
    println!("Enter the operation you want to do:\n1. Addition");
    println!("2. Subtraction\n3. Multiplication\n4. Division\n5. Exit");
    io::stdin().read_line(&mut op).expect("Invalid Input");

    let op: i32 = match op.trim().parse() {
        Ok(num)=> num, Err(_)=>{
            println!("Invalid Choice!");
            return;
        }
    };


}
