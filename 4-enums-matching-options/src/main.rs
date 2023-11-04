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

fn type_of_day(day:&Day) -> &str {
    let resp:&str = match day {
        Day::Monday | Day:: Tuesday => "Boring days",
        Day::Wednesday | Day::Thrusday => "Exhaustive & waiting for weekend days",
        Day::Friday => "Happy workday",
        _ => "Sleeping weekend" // This is like default, we must always have a default for match
    };
    resp
}

fn main() {
    let today = Day::Wednesday; 
    println!("It's {:?}, there it's: {:?}",today, type_of_day(&today))
}

