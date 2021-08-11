// mutable fields of immutable structs are not supported
#[derive (Debug)]   // To allow fine-print
struct User {
    username: String,   // Because &str needs explicit lifetime parameter
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs, no names for fields
struct Point(i32, i32, i32);

impl User {
    fn build(email: String, username: String) -> User {
        User {
    // No need to repeat fields in case values have the same name
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    fn print(&self, fine: bool) {
//        let format: &str = if fine {"{:#?}"} else {"{:?}"};
//        println!(format, self);
        if fine {
            println!("{:#?}", self);
        } else {
            println!("{:?}", self);
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1 == {:?}", user1);

    user1.email = String::from("anotheremail@example.com");
    println!("user1 == {:?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
// Other fields will be inherited from user1
        ..user1
    };
    println!("user2 == {:#?}", user2);  // Even finer print

    let origin = Point(0, 0, 0);
    println!("origin == ({}, {}, {})", origin.0, origin.1, origin.2);   // No #[derive(Debug)], no easy fine-print
    
    let user3 = User::build(String::from("XXX"), String::from("MiName"));
    user3.print(false);
    user3.print(true);
}

fn build_user(email: String, username: String) -> User {
    User {
// No need to repeat fields in case values have the same name
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

