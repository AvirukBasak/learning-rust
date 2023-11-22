use super::helperfn::get_filename;
use super::helperfn::input;

pub fn test() {
    let ip: i32 = input(&format!("{}: Enter a num: ", get_filename(file!())));
    println!("{}: {}", get_filename(file!()), ip);
    let ip: String = input(&format!("{}: Enter an str: ", get_filename(file!())));
    println!("{}: {}", get_filename(file!()), ip);
}
