use super::helperfn::get_filename;
use super::helperfn::input;

pub fn cyl_vol() {
    const PI: f32 = 3.14;
    let rad: f32 = input(&format!(
        "{}: Enter radius of cylinder: ",
        get_filename(file!())
    ));
    let height: f32 = input(&format!(
        "{}: Enter height of cylinder: ",
        get_filename(file!())
    ));
    let vol = PI * height * rad.powf(2.0);
    println!("{}: Cylinder volume = {}", get_filename(file!()), vol);
}
