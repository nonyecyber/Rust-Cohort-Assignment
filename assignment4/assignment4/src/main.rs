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
}
