use std::io;
fn main() {
    /*-----------------------------------------------------------------------------------------------------
    This code calculates the grades of students
    --------------------------------------------------------------------------------------------- */
    println!("Let us calculate grades!");
//Asks the user for the number of students(minimum of ten)
    let num_students = loop {
        println!("How many students? (minmum of 10) ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    match input.trim().parse::<u32>() {
        Ok(n) if n >= 10 => break n,
        Ok(_) => println!("Please enter a number greater than or equal to 10"),
        Err(_) => println!("Please enter a valid number"),
    }
    };
    //Asks for name for each student
    for i in 1..=num_students {
        println!("Student {}", i);
        println!("Enter student's name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name)
        .expect("Failed to read name");
    let name = name.trim().to_string();
        //Assigns a score to each student
    let score = loop {
        println!("Enter score (0-100) for {}:", name);
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .expect("Failed to read score");
      //Ensures the score is between 0 and 100  
    match input.trim().parse::<f32>() {
        Ok(s) if s >= 0.0 && s <= 100.0 => break s,
        Ok(_) => println!("Score must be beyween 0 and 100"),
        Err(_) => println!("Please enter a valid number"),
    }
    };
    //Assigns a grade to a range of score
    let grade = if score >= 80.0 {
        "A"
    } else if score >= 70.0 {
        "B"
    } else if score >= 60.0 {
        "C"
    } else if score >= 50.0 {
        "D"
    } else {
        "F"
    };
    //Prints student name, score and grade
    println!("{}'s score: {:.1} - Grade: {}", name, score, grade);
    }
}
    
    

