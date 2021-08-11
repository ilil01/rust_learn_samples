enum IpAddr {
    V4(u8, u8, u8, u8),     // So not to mess with struct
    V6(String),
}

impl IpAddr {
    fn print(&self) {
        match self {
            IpAddr::V4 => println!("{}:{}:{}:{}", self.);
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
