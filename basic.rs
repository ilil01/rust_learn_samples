use std::io;    // to be able to process input
use rand::Rng;  // to generate random numbers. Using such crates require cargo
use std::cmp::Ordering; // Less, Greater, Equal

fn main() {
    println!("Hello, world!");      //macro invoked
    let secret = rand::thread_rng() // random number, local to thred and seeded by OS
        .gen_range(1, 101);
    loop {  // endless loop
        println!("Guess the number:");  //<function> nope, also macro but not invoked
        let mut guess = String::new();  // declares (let) a mutable (mut) variable, binding it to new instance of String
        io::stdin()                     // std::io::stdin without `use`
            .read_line(&mut guess)      // reading input, String must be mutable
            .expect("FAIL");            // read_line returns Result, which is enum; if Err, prints that
        let guess: u32 = match guess.trim()   // shadow previous declaration, like in LISP; eliminate whitespaces
            .parse()                    // parsing to a number, variety of types so hint after variable name
            {
                Ok(num) => num,         // if `parse` returned successfully, Result is Ok containing actual result
                Err(_) => continue      // else just retry
            };
//        println!("Your guess is :{}, and there's a 5 :{}", guess, 7);
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too little!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Gotcha!");
                break;
            },
        }
    }

    for i in 1..4 {
        println!("{}", i);
//        let tmp = [9; i]; // non-constant values not supported
//        println!("{}", tmp);
    }
    for i in (1..4).rev() {
        println!("{}", i);
    }
}

// Ok, you see
/* there actually ARE block comments!
 * Here they WORK!
 */
/// Also special comments used to build the HTML doc
/// about your not so great crate/package/whatever!
/// And this comment SHOULD comment some item!
/// rustc forces it!
fn do_nothing() {
}
