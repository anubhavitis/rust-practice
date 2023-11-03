// Testing some of the basic rust data types, annotations, conversions etc.

// function copied from chatgpt to print type of a variable
fn type_of<T: std::fmt::Debug>(value: T) {
    println!("Type: {} Value: {:?}", std::any::type_name::<T>(), value);
}


fn main() {
    // defaults
    let my_int = 123; // i32 integer
    type_of(my_int);
    let my_string = "hell"; // &str string. Notice how this is address type
    type_of(my_string);
    let my_float = 1.23; // f64 float type
    type_of(my_float);


    // type annotation
    let my_int2: i64 = 123; // Now this is i64
    type_of(my_int2);
    let my_int3 = 123i64; // This is also i64
    type_of(my_int3);
}