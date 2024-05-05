use std::io;

fn main() {
    let mut x: String = String::new();
    let mut y: String = String::new();
    let mut op: String = String::new();
    let mut res: f64 = 0.0;

    println!("Enter the first number: ");
    io::stdin().read_line(&mut x).expect("Invalid Input");

    let x: f64 = match x.trim().parse() {
        Ok(num) => num, 
        Err(_)=> {
            println!("Invalid first number!");
            return;
        }
    };

    println!("Enter the second number: ");
    io::stdin().read_line(&mut y).expect("Invalid Input");

    let y: f64 = match y.trim().parse() {
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
    match op {
        1 => res = add(x, y),
        2 => res = sub(x, y),
        3 => res = mul(x, y),
        4 => res = div(x, y), 
        5=> {
        }, _ => {
            println!("Invalid Selection!");
            return;
        }
    }

    println!("Result: {}", res);
}

fn add (x: f64, y: f64) -> f64{
    return x + y;
}

fn sub (x: f64, y: f64) -> f64{
    return x - y;
}

fn mul (x: f64, y: f64) -> f64{
    return x * y;
}

fn div (x: f64, y: f64) -> f64{
    return x / y;
}