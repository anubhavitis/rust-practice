#![allow(dead_code)]

// Enums are way to declare values of similar type.
// Like this, we can create stuff with Day keyword
// and can use any value inside day to work on.
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thrusday,
    Friday,
    Saturday,
    Sunday
}

fn type_of_day(day:&Day) -> Option<&str> {
    //Option<T> stands for some value of type T, or None.
    let resp:Option<&str> = match day {
        Day::Monday | Day:: Tuesday => Some("Boring days"),
        Day::Wednesday | Day::Thrusday => Some("Exhaustive & waiting for weekend days"),
        Day::Friday => Some("Happy workday"),
        _ => None // This is like default, we must always have a default for match.
    };
    resp
}

fn test_option() {
    let today = Day::Sunday; 
    
    let day_type = type_of_day(&today);
    match day_type {
        Some(s) => println!("It's {:?}, there it's: {:?}",today, s),
        None => println!("It's {:?}, there it's: {:?}",today, "Weekend. Go to sleep."),
    }
}


fn main() {
    // Uncomment the part of code that you want to test.
    
    // test_option();
    test_result();
}

// #############################################################################
// For simplicity, I have implemented the test code for Result<T,E> below main()

// function copied from chatgpt to print type of a variable
fn type_of<T: std::fmt::Debug>(value: T) {
    println!("Type: {} Value: {:?}", std::any::type_name::<T>(), value);
}

fn test_result() {
    // explicit annotation used here, since the default is i32
    let tests:Vec<(u32,u32)> = vec![(4, 5), (7, 5), (10, 5)];

    for (a,b) in tests.iter() {
        let resp = divide(*a,*b);
        print!("{} divided by {} is: ", a, b);
        match resp {
            Err(err) => println!("Error : {}", err),
            Ok(val) => println!("{}", val),
        }
    }
}


fn divide(a:u32, b:u32) -> Result< u32, String> {
    let resp:Result<u32, String> ;
    if b==0 {
       resp= Err("Denominator must be non-zero".to_string());
    }
    else if a<b {
       resp= Err ("Numerator is less than denominator".to_string());
    }
    else {
       resp= Ok(a/b);
    }
    resp
}