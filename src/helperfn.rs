use std::io::{self, Write};
use std::path::Path;

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

pub fn get_filename(filepath: &str) -> &str {
    Path::new(filepath)
        .file_name()
        .and_then(|s| s.to_str())
        .expect(stringify!(error getting filename))
}
