enum IpAddr {
    V4(u8, u8, u8, u8),     // So not to mess with struct
    V6(String),
    NonValid,
}

impl IpAddr {
    fn print(&self) {
        match self {
            IpAddr::V4(a1, a2, a3, a4) => println!("{}:{}:{}:{}", a1, a2, a3, a4),
            IpAddr::V6(s)              => println!("{}", s),
            _                          => println!("Invalid IP!")   /* marked unused, other names could be used */
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    home.print();
    loopback.print();

    let inval = IpAddr::NonValid;
    inval.print();

    if let IpAddr::V4(a1, a2, a3, a4) = inval {
        println!{"{}", a1 + a2 + a3 + a4};
    }
    if let IpAddr::V4(a1, a2, a3, a4) = home {
        println!{"{}", a1 + a2 + a3 + a4};
    }
}
