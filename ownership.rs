fn main() {
    let x: i32 = 5;
    let y: i32 = x;  // stack value, just copy
    let x: String = String::from("hello");
    let y: String = x;  // heap value, shallow copy + end of life for x
    let x: String = y.clone();   // deep copy
    let y = take_own(y);    // func takes ownership of y, so it's no more a valid String
    let z = no_take_own(&x);// references avoid ownership, so x continue to be valid. `&mut x` for mutable reference
    println!("x{}, y{}, z{}", x, y, z);
}

fn take_own (a: String) -> usize {
    a.len()
}

fn no_take_own (a: &String) -> usize {
    a.len()
}
