use super::helperfn::get_filename;
use super::helperfn::input;

pub fn interest() {
    let prin: f32 = input(&format!(
        "{}: Enter principal (Rs): ",
        get_filename(file!())
    ));
    let time: f32 = input(&format!("{}: Enter period (y): ", get_filename(file!())));
    let rate: f32 = input(&format!("{}: Enter rate (%): ", get_filename(file!())));
    let si = prin * time * rate / 100.0;
    println!("{}: SI = {}", get_filename(file!()), si);
}
