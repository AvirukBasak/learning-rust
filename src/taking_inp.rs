use super::helperfn::get_filename;
use std::io::{self, Write};

pub fn input<T>(prompt: &str) -> T
where
    T: std::str::FromStr,
{
    print!("{}", prompt);
    io::stdout().flush().expect("prompt output error");
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    match buf.trim().to_owned().parse::<T>() {
        Ok(i) => i,
        Err(_) => panic!("invalid input"),
    }
}

pub fn test() {
    let ip: i32 = input(&format!("{}: Enter a num: ", get_filename(file!())));
    println!("{}: {}", get_filename(file!()), ip);
    let ip: String = input(&format!("{}: Enter an str: ", get_filename(file!())));
    println!("{}: {}", get_filename(file!()), ip);
}
