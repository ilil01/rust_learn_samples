fn main() {
// You can only have either one mutable reference or many immutable
    let s = "Hello, world!";
    let hello = &s[..6];
    let world = &s[8..];
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let zero2five = &arr[..5];
    let five2ten = &arr[5..];
    println!("s == {}", s);
    println!("hello world == {} {}", hello, world);
    print!("arr == [");
    for elem in arr.iter() {
        print!("{}, ", elem);
    }
    println!("]");
    print!("zero2five == [");
    for elem in zero2five.iter() {
        print!("{}, ", elem);
    }
    println!("]");
    print!("five2ten == [");
    for elem in five2ten.iter() {
        print!("{}, ", elem);
    }
    println!("]");
}
