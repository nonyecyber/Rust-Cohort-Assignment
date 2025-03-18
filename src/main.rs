fn main() {
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
}
