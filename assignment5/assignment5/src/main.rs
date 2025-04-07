use std::io;

fn main() {
    println!("My CLI calculator");
    println!("please enter the first number");

    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed");
    
    println!("please enter the second number");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed");
    
    println!("please enter the operator");
    let mut operator = String::from("+ - * /");
    io::stdin().read_line(&mut operator).expect("Failed");
    
    let num1: f64 = match num1.parse(){
        Ok(n) => n,
        Err(_) => 0.0
    };
    let num2: f64 = match num2.parse(){
        Ok(n) => n,
        Err(_) => 0.0
    };
    
    let result = calculate(num1, num2, &operator);
    match calculate(num1, num2, &operator){
        Ok (result) => println!("Result: {}", result),
            Err(_) => println!("Failed to calculate")
        }
}

fn calculate(num1:f64, num2:f64, operator: &str)->Result<f64, String>{
    match operator{
    "+" => Ok(num1 + num2),
    "-" => Ok(num1 - num2),
    "*" => Ok(num1 * num2),
    "/" => {
        if num2 == 0.0 {
           Err("Division cannot be by 0".to_string())
        }
        else
        {
           Ok(num1/num2)
        }
    }  
    _=> Err("Invalid operator".to_string()),
    }
}