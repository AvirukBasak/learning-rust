use super::helperfn::get_filename;
use std::io::{self, Write};

pub fn input<T: std::str::FromStr>(prompt: &str) -> T {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    let buf = String::from(buf.trim());
    let ret = if let Ok(i) = buf.parse::<T>() {
        Some(i)
    } else {
        None
    };
    return ret.expect("invalid type");
}

pub fn test() {
    let ip: i32 = input(&format!("{}: Enter a num: ", get_filename(file!())));
    println!("{}: {}", get_filename(file!()), ip);
    let ip: String = input(&format!("{}: Enter an str: ", get_filename(file!())));
    println!("{}: {}", get_filename(file!()), ip);
}
