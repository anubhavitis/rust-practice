
fn find_index(a:&String) -> &str {
    let x = a.as_bytes();

    for (i, &iter) in x.iter().enumerate() {
        if iter == b' ' {
            // here we are returning a slice that is indexed to reference of a:String
            return &a[..i];
        }
    }
    // here we are returning a slice that is indexed to reference of a:String
    &a[..]
}

fn main() {
    let mut s = "hello world".to_string();
    // The x that we recieve below is a slice that is indexed to reference of s:String
    let x= find_index(&s);
    println!("{}", x);

    // Try this statement before line-19 println. It'll give error. 
    // Because x is referenced to s, you can't clear s and use x later., 
    s.clear();
}