// Testing some of the basic rust data types, annotations, conversions etc.

// function copied from chatgpt to print type of a variable
fn type_of<T: std::fmt::Debug>(value: T) {
    println!("Type: {} Value: {:?}", std::any::type_name::<T>(), value);
}

fn default() {
    // defaults
    let my_int = 123; // i32 integer
    type_of(my_int);
    let my_string = "hell"; // &str string. Notice how this is address type
    type_of(my_string);
    let my_float = 1.23; // f64 float type
    type_of(my_float);
}

fn immutable_type_annotations() {
    // type annotation
    let my_int2: i64 = 123; // Now this is i64
    type_of(my_int2);
    let my_int3 = 123i64; // This is also i64
    type_of(my_int3);

    // All the above varibles are immutable by default. ie. their values can't be edited.
    // my_int += 1; This will give error: cannot assign twice to immutable variable
}

fn mutable_variables(){
    let mut my_mut_int = 0; // Here mut keyword is to denote that the variable is mutable
    my_mut_int += 1;
    type_of(my_mut_int);

}

fn type_inference_as_used(){
    let mut my_int = 123; // This will taken by default as i32
    my_int = 132i64; // But the actual type will be inferred from here
    type_of(my_int);

    // ----------------- Another example ----------------

    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;
    type_of(elem);

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem); // This will let the compiler know that it's vec<u8>
    type_of(vec)
}

fn main() {
    // Uncomment the following funcs as you want to try it out.


    // default();
    // immutable_type_annotations();
    // mutable_variables();
    type_inference_as_used();
}