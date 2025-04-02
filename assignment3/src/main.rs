use std::io;

fn main() {
    let gifts = ["A partridge in a pear tree", "Two turtle-doves", "Three french hens", "Four calling birds", "Five gold rings", "Six geese a laying", "Seven swans a swimming", "Eight maids a milking", "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping", "Twelve drummers drumming"];
    for day in 0..12{
        println!("On the {} day of christmas, my true love gave to me",
        match day {
            0 => "first",
            1 => "second",
            2 => "third",
            3 => "fourth",
            4 => "fifth",
            5 => "sixth",
            6 => "seventh",
            7 => "eighth",
            8 => "ninth",
            9 => "tenth",
            10 => "eleventh",
            11 => "twelfth",
            _ => unreachable!()
        }
    );
    for gift in (0..=day).rev() {
        if gift == 0 && day != 0 {
            print!("And ");
        }
        println!("{}", gifts[gift]);
    }
    println!();
    }
    println!("Let us calculate grades!");

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
    for i in 1..=num_students {
        println!("Student {}", i);
        println!("Enter student's name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name)
        .expect("Failed to read name");
    let name = name.trim().to_string();

    let score = loop {
        println!("Enter score (0-100) for {}:", name);
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .expect("Failed to read score");

    match input.trim().parse::<f32>() {
        Ok(s) if s >= 0.0 && s <= 100.0 => break s,
        Ok(_) => println!("Score must be beyween 0 and 100"),
        Err(_) => println!("Please enter a valid number"),
    }
    };

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
    
    println!("{}'s score: {:.1} - Grade: {}", name, score, grade);
    }
}
    
    

