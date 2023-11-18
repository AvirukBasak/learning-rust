use std::io::{self, Write};

pub fn input<T: std::str::FromStr>(prompt: &str) -> T {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    let buf = String::from(buf.trim());
    let ret = if let Ok(i) = buf.parse::<T>() { Some(i) } else { None };
    return ret.expect("invalid type");
}

pub fn interest() {
    let ip: i32 = input("enter a num: ");
    println!("{}", ip);
    let ip: String = input("enter a str: ");
    println!("{}", ip);
}
