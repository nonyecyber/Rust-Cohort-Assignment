
fn main()
{
    /*----------------------------------------------------------------------------------------------
    *This function:
    *-Prints name
    *-Prints matriculation number
    *-Prints my favourite emoji
    *-Prints the result of the division of two floats
    *-Prints other functions
    --------------------------------------------------------------------------------------------- */
    let name : &str = "Ogbonna Chinonye";
    let matno : &str = "ENG2204014";
    let fave : &str = ".\u{2764}";
    let x : f32 = 2.0;
    let y : f32 = 3.0;
    myname(name);
    mymat(matno);
    emoji(fave);
    divide(x, y);
    {
        println!("My name is {}.", name);//Full name
        println!("My matriculation number is {}.", matno);//Matriculation number
        println!("My favourite emoji is {}.", fave);//Favourite emoji
        println!("The value of {} divided by {} is {}.", x, y, x / y);//Division of two floats
    }
    //Full name function
    fn myname(girlnam : &str){
        println!("My name is {}.", girlnam);
    }
    //Matriculation number function
    fn mymat(mat : &str){
        println!("My matriculation number is {}.", mat);
    }
    //Favourite emoji function
    fn emoji(emo : &str){
        println!("My favorite emoji is {}.", emo);
    }//Division function
    fn divide(a : f32, b : f32){
        println!("The value of {} divided by {} is {}.", a, b, a / b);
    }
    
    let scores1: [i32; 3] = [85, 90, 21];
    let student1: (&str, u32, bool) = ("Angel", 21, true);
    let _scores2: [i32; 3] = [84, 35, 93];
    let student2: (&str, u32, bool) = ("Praise", 34, false);
    let courses:[&str; 3] = ["CPE591", "CPE491", "CPE391"];
    println!("This is the information for {} in CPE591 is {}, 491 is {} and 392 is {}.", student1.0, scores1[0], scores1[1], scores1[2]);
    println!("This is the information for {} in {} is 84, {} is 35 and {} is 93.", student2.0, courses[0], courses[1], courses[2]);
    let num: [i32;3] = [84, 35, 93];
    let sum: i32 = num.iter().sum();
    let length = num.len() as f32;
    let average = sum as f32/length;
    println!("The average of {}, {} and {} is {}", num[0], num[1], num[2], average);
    struct Rectangle{
        length : u32,
        breadth : u32,
    }
    {
    let rect1 = Rectangle{
        length : 30,
        breadth : 40,};
        println!("The area of the rectangle is {}.", area(&rect1));}
    fn area(rectangle : &Rectangle)-> u32{
        rectangle.length * rectangle.breadth
    }
    {
    enum Shape{
        Circle(f64),
        Rectangle(f64, f64),
    }
    fn area(shape : Shape)->f64{
        match shape{
            Shape::Circle(r)=> 3.14 * r * r,
            Shape::Rectangle(w, h) => w * h,
        }
    }
    {
        let my_shape = Shape::Circle(5.0);
        let my_other_shape = Shape::Rectangle(6.3, 5.9);
        println!("Area = {}", area(my_shape));
        println!("Area = {}", area(my_other_shape));
    }
{
    //Create struct for students record
struct Student{
    name:String,
    score:u32,
}
//Store 10 students record
let students=vec![
Student{
    name:String::from("Anna"),
    score:98,},
Student{
    name:String::from("Ben"),
    score:88,
},
Student{
    name:String::from("Cain"),
    score:78,
},
Student{
    name:String::from("David"),
    score:68,
},
Student{
    name:String::from("Ella"),
    score:58,
},
Student{
    name:String::from("Favour"),
    score:48,
},
Student{
    name:String::from("George"),
    score:38,
},
Student{
    name:String::from("Hannah"),
    score:28,
},
Student{
    name:String::from("Ian"),
    score:18,
},
Student{
    name:String::from("Jack"),
    score:8,
},
];
let total: u32 = students.iter().map(|s|s.score).sum();//Calculate total score
let average: f32 = total as f32/students.len() as f32;//Calculate average score
//Determine highest scoring student
let mut highest = &students[0];
for student in &students{
    if student.score > highest.score{
        highest=student;
    }
}//Determine the number of students who passed and those who failed
let mut passed = 0;
let mut failed = 0;
for student in &students{
    if student.score >= 50{
        passed += 1;
    }else{
        failed += 1;
    }
}
//Print the output
println!("Total score of all students is {}", total);
println!("The average sore is {:.2}", average);
println!("The highest scoring student is {} with a score of {}", highest.name, highest.score);
println!("Number of students who passed: {}", passed);
println!("Number of students who failed: {}", failed);
}
}
    }

