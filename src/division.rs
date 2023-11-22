use super::helperfn::input;
use super::helperfn::get_filename;

pub fn division() {
    let x: i32 = input(&format!("{}: Enter 1st integer: ", get_filename(file!())));
    let y: i32 = input(&format!("{}: Enter 2nd integer: ", get_filename(file!())));
    let q = x / y;
    let r = x % y;
    println!("{}: Result = {q}, {r}", get_filename(file!()))
}
