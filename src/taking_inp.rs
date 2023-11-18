use std::io::{self, Write};

pub fn input<T: std::str::FromStr>(prompt: &str) -> T {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let ret: T = {
        let mut buf: String = String::new();
        if let Ok(_) = io::stdin().read_line(&mut buf) {
            let buf = String::from(buf.trim());
            if let Ok(i) = buf.parse::<T>() { Some(i) } else { None }}
        else { None }
    }.expect("invalid input type");
    return ret;
}

pub fn interest() {
    let ip: i32 = input("enter a num: ");
    println!("{}", ip);
    let ip: String = input("enter a str: ");
    println!("{}", ip);
}
