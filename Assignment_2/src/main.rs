/*----------------------------------------------------------------------------------------------
This is a simple CLI calculator. It can:
*Perform addition, subtraction, multiplication and division.
*Calculate CGPA
----------------------------------------------------------------------------------------- */
use std::env;
use std::process;
use std::io;


fn print_usage(){
    println!("Usage: calculator <num1> <operator> <num2>");
    println!("Operators; +, -, *, /");
}//Match operators to expected output
fn calculate(num1: f64, operator: &str, num2: f64)->Result<f64, String>{
    match operator{
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0{
                Err("Division by zero".to_string())
            }else{
                Ok(num1/num2)
            }
        }
        _ => Err("Invalid operator".to_string()),
    }
}
fn main() {
    //Input is gotten as command line argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 4{
        print_usage();
        process::exit(1);
    }
    let num1: f64 = match args[1].parse(){
        Ok(n) => n,
        Err(_) =>{
            eprintln!("Error: Invalid number'{}'", args[1]);
            process::exit(1);
        }
    };
    let operator = &args[2];

    let num2: f64 = match args[3].parse() {
     Ok(n) => n,
     Err(_) =>{
        eprintln!("Error: Invalid number '{}'", args[3]);
        process::exit(1);
     }
    };
    //Print result of addition, subtraction, multiplication and division
    match calculate(num1, operator, num2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        
    }
    //Input number of subjects
    println!("To calculate CGPA, enter number of subjects: ");

    let mut num_subjects = String::new();
    io::stdin().read_line(&mut num_subjects).expect("Failed to read input");

    let num_subjects: usize = match num_subjects.trim().parse()  {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Please enter a valid number");
            return;
        }
    };
    if num_subjects == 0{
        eprintln!("Error: Number of subjects cannon be zero");
        return;
    }//Input grade points and total credits
    let mut total_grade_points = 0.0;
    let mut total_credits = 0.0;

    for i in 1..=num_subjects{
        println!("Enter grade point for subject {}:", i)
    };
    let mut grade_point = String::new();
    io::stdin().read_line(&mut grade_point).expect("Failed to read input");

    let grade_point: f64 = match grade_point.trim().parse(){
        Ok(g) if g >= 0.0 && g <= 5.0 =>g,
        _=>{
            eprintln!("Error: Please enter a valid grade point(0.0 - 5.0)");
            return;
        }
    };
    println!("Enter credit score for subject:");
    let mut credit_score = String::new();
    io::stdin().read_line(&mut credit_score).expect("Failed to read input");
    let credit_score: f64 = match credit_score.trim().parse(){
        Ok(c) if c > 0.0 => c,
        _=>{
            eprintln!("Error: Please enter a valid credit score (greater than zero)");
            return;
        }
    };
    total_grade_points +=grade_point * credit_score;
    total_credits += credit_score;
    //Prints the calculated CGPA
    let cgpa = total_grade_points/total_credits;
    println!("\n Your CGPA is:{:.2}", cgpa);

    }


